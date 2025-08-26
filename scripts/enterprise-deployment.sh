#!/bin/bash

# Enterprise NestGate Deployment Script
# Automated deployment for production enterprise environments

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEPLOYMENT_ENV="${DEPLOYMENT_ENV:-production}"
CLUSTER_NAME="${CLUSTER_NAME:-nestgate-enterprise}"
NAMESPACE="${NAMESPACE:-nestgate-enterprise}"
REPLICAS="${REPLICAS:-3}"
STORAGE_SIZE="${STORAGE_SIZE:-1Ti}"
BACKUP_ENABLED="${BACKUP_ENABLED:-true}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking deployment prerequisites..."
    
    local missing_tools=()
    
    # Check required tools
    for tool in kubectl helm docker podman jq yq; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        log_info "Please install missing tools and try again"
        exit 1
    fi
    
    # Check Kubernetes connection
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster"
        log_info "Please ensure kubectl is configured correctly"
        exit 1
    fi
    
    # Check Helm
    if ! helm version &> /dev/null; then
        log_error "Helm is not properly configured"
        exit 1
    fi
    
    log_success "Prerequisites check completed"
}

# Create namespace and RBAC
setup_namespace() {
    log_info "Setting up namespace and RBAC..."
    
    # Create namespace
    kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -
    
    # Create service account
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ServiceAccount
metadata:
  name: nestgate-enterprise
  namespace: $NAMESPACE
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: nestgate-enterprise
rules:
- apiGroups: [""]
  resources: ["pods", "services", "endpoints", "persistentvolumeclaims", "events", "configmaps", "secrets"]
  verbs: ["*"]
- apiGroups: ["apps"]
  resources: ["deployments", "daemonsets", "replicasets", "statefulsets"]
  verbs: ["*"]
- apiGroups: ["monitoring.coreos.com"]
  resources: ["servicemonitors"]
  verbs: ["get", "create"]
- apiGroups: ["networking.k8s.io"]
  resources: ["networkpolicies"]
  verbs: ["*"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: nestgate-enterprise
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: nestgate-enterprise
subjects:
- kind: ServiceAccount
  name: nestgate-enterprise
  namespace: $NAMESPACE
EOF
    
    log_success "Namespace and RBAC configured"
}

# Deploy monitoring stack
deploy_monitoring() {
    log_info "Deploying monitoring stack..."
    
    # Add Prometheus Helm repository
    helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
    helm repo add grafana https://grafana.github.io/helm-charts
    helm repo update
    
    # Deploy Prometheus
    helm upgrade --install prometheus prometheus-community/kube-prometheus-stack \
        --namespace "$NAMESPACE" \
        --create-namespace \
        --set prometheus.prometheusSpec.retention=30d \
        --set prometheus.prometheusSpec.storageSpec.volumeClaimTemplate.spec.resources.requests.storage=100Gi \
        --set grafana.adminPassword="$(openssl rand -base64 32)" \
        --set grafana.persistence.enabled=true \
        --set grafana.persistence.size=20Gi \
        --wait
    
    log_success "Monitoring stack deployed"
}

# Deploy storage infrastructure
deploy_storage() {
    log_info "Deploying storage infrastructure..."
    
    # Create storage class for high-performance storage
    cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: nestgate-fast-ssd
  annotations:
    storageclass.kubernetes.io/is-default-class: "false"
provisioner: kubernetes.io/aws-ebs
parameters:
  type: gp3
  iops: "16000"
  throughput: "1000"
  encrypted: "true"
allowVolumeExpansion: true
reclaimPolicy: Retain
volumeBindingMode: WaitForFirstConsumer
EOF
    
    # Create persistent volume claims
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: nestgate-data
  namespace: $NAMESPACE
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: nestgate-fast-ssd
  resources:
    requests:
      storage: $STORAGE_SIZE
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: nestgate-backup
  namespace: $NAMESPACE
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: nestgate-fast-ssd
  resources:
    requests:
      storage: 500Gi
EOF
    
    log_success "Storage infrastructure deployed"
}

# Create configuration
create_configuration() {
    log_info "Creating enterprise configuration..."
    
    # Create ConfigMap with enterprise configuration
    kubectl create configmap nestgate-config \
        --from-file="$PROJECT_ROOT/config/enterprise-production.toml" \
        --namespace="$NAMESPACE" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create secrets
    kubectl create secret generic nestgate-secrets \
        --from-literal=cluster-secret="$(openssl rand -base64 32)" \
        --from-literal=encryption-key="$(openssl rand -base64 32)" \
        --from-literal=database-password="$(openssl rand -base64 32)" \
        --namespace="$NAMESPACE" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create TLS certificates (self-signed for demo)
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout /tmp/tls.key \
        -out /tmp/tls.crt \
        -subj "/CN=nestgate-enterprise/O=nestgate-enterprise"
    
    kubectl create secret tls nestgate-tls \
        --cert=/tmp/tls.crt \
        --key=/tmp/tls.key \
        --namespace="$NAMESPACE" \
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Cleanup temporary files
    rm -f /tmp/tls.key /tmp/tls.crt
    
    log_success "Configuration created"
}

# Deploy NestGate application
deploy_application() {
    log_info "Deploying NestGate enterprise application..."
    
    # Create StatefulSet for NestGate
    cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: nestgate-enterprise
  namespace: $NAMESPACE
  labels:
    app: nestgate-enterprise
spec:
  serviceName: nestgate-enterprise-headless
  replicas: $REPLICAS
  selector:
    matchLabels:
      app: nestgate-enterprise
  template:
    metadata:
      labels:
        app: nestgate-enterprise
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9090"
        prometheus.io/path: "/metrics"
    spec:
      serviceAccountName: nestgate-enterprise
      securityContext:
        fsGroup: 1000
        runAsUser: 1000
        runAsNonRoot: true
      containers:
      - name: nestgate
        image: nestgate-enterprise:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8443
          name: https
          protocol: TCP
        - containerPort: 8444
          name: cluster
          protocol: TCP
        - containerPort: 9090
          name: metrics
          protocol: TCP
        env:
        - name: NESTGATE_CONFIG
          value: /etc/nestgate/enterprise-production.toml
        - name: NESTGATE_CLUSTER_NAME
          value: "$CLUSTER_NAME"
        - name: NESTGATE_NODE_ID
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        - name: NESTGATE_CLUSTER_SECRET
          valueFrom:
            secretKeyRef:
              name: nestgate-secrets
              key: cluster-secret
        volumeMounts:
        - name: config
          mountPath: /etc/nestgate
          readOnly: true
        - name: data
          mountPath: /data/nestgate
        - name: backup
          mountPath: /backup/nestgate
        - name: tls
          mountPath: /etc/nestgate/certs
          readOnly: true
        resources:
          requests:
            cpu: 2000m
            memory: 4Gi
          limits:
            cpu: 8000m
            memory: 16Gi
        livenessProbe:
          httpGet:
            path: /health/live
            port: 8443
            scheme: HTTPS
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health/ready
            port: 8443
            scheme: HTTPS
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        startupProbe:
          httpGet:
            path: /health/startup
            port: 8443
            scheme: HTTPS
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 30
      volumes:
      - name: config
        configMap:
          name: nestgate-config
      - name: tls
        secret:
          secretName: nestgate-tls
      - name: data
        persistentVolumeClaim:
          claimName: nestgate-data
      - name: backup
        persistentVolumeClaim:
          claimName: nestgate-backup
  updateStrategy:
    type: RollingUpdate
    rollingUpdate:
      partition: 0
EOF
    
    # Create headless service for StatefulSet
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Service
metadata:
  name: nestgate-enterprise-headless
  namespace: $NAMESPACE
  labels:
    app: nestgate-enterprise
spec:
  clusterIP: None
  selector:
    app: nestgate-enterprise
  ports:
  - port: 8443
    name: https
  - port: 8444
    name: cluster
  - port: 9090
    name: metrics
EOF
    
    # Create load balancer service
    cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Service
metadata:
  name: nestgate-enterprise
  namespace: $NAMESPACE
  labels:
    app: nestgate-enterprise
  annotations:
    service.beta.kubernetes.io/aws-load-balancer-type: nlb
    service.beta.kubernetes.io/aws-load-balancer-scheme: internal
spec:
  type: LoadBalancer
  selector:
    app: nestgate-enterprise
  ports:
  - port: 443
    targetPort: 8443
    name: https
    protocol: TCP
EOF
    
    log_success "NestGate application deployed"
}

# Deploy ingress
deploy_ingress() {
    log_info "Deploying ingress controller..."
    
    # Install NGINX ingress controller
    helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
    helm repo update
    
    helm upgrade --install ingress-nginx ingress-nginx/ingress-nginx \
        --namespace ingress-nginx \
        --create-namespace \
        --set controller.service.type=LoadBalancer \
        --set controller.metrics.enabled=true \
        --set controller.podAnnotations."prometheus\.io/scrape"=true \
        --set controller.podAnnotations."prometheus\.io/port"=10254 \
        --wait
    
    # Create ingress for NestGate
    cat <<EOF | kubectl apply -f -
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: nestgate-enterprise
  namespace: $NAMESPACE
  annotations:
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/backend-protocol: "HTTPS"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - nestgate.example.com
    secretName: nestgate-tls-ingress
  rules:
  - host: nestgate.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: nestgate-enterprise
            port:
              number: 443
EOF
    
    log_success "Ingress controller deployed"
}

# Setup monitoring and alerting
setup_monitoring() {
    log_info "Setting up monitoring and alerting..."
    
    # Create ServiceMonitor for Prometheus
    cat <<EOF | kubectl apply -f -
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: nestgate-enterprise
  namespace: $NAMESPACE
  labels:
    app: nestgate-enterprise
spec:
  selector:
    matchLabels:
      app: nestgate-enterprise
  endpoints:
  - port: metrics
    interval: 30s
    path: /metrics
EOF
    
    # Create PrometheusRule for alerts
    cat <<EOF | kubectl apply -f -
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: nestgate-enterprise-alerts
  namespace: $NAMESPACE
  labels:
    app: nestgate-enterprise
spec:
  groups:
  - name: nestgate.rules
    rules:
    - alert: NestGateDown
      expr: up{job="nestgate-enterprise"} == 0
      for: 1m
      labels:
        severity: critical
      annotations:
        summary: "NestGate instance is down"
        description: "NestGate instance {{ \$labels.instance }} has been down for more than 1 minute."
    
    - alert: NestGateHighCPU
      expr: rate(cpu_usage_seconds_total{job="nestgate-enterprise"}[5m]) > 0.8
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "NestGate high CPU usage"
        description: "NestGate instance {{ \$labels.instance }} has high CPU usage: {{ \$value }}%"
    
    - alert: NestGateHighMemory
      expr: memory_usage_percent{job="nestgate-enterprise"} > 80
      for: 5m
      labels:
        severity: warning
      annotations:
        summary: "NestGate high memory usage"
        description: "NestGate instance {{ \$labels.instance }} has high memory usage: {{ \$value }}%"
    
    - alert: NestGateStorageFull
      expr: storage_usage_percent{job="nestgate-enterprise"} > 90
      for: 1m
      labels:
        severity: critical
      annotations:
        summary: "NestGate storage almost full"
        description: "NestGate instance {{ \$labels.instance }} storage is {{ \$value }}% full"
EOF
    
    log_success "Monitoring and alerting configured"
}

# Setup backup system
setup_backup() {
    if [ "$BACKUP_ENABLED" != "true" ]; then
        log_info "Backup is disabled, skipping backup setup"
        return
    fi
    
    log_info "Setting up backup system..."
    
    # Create backup CronJob
    cat <<EOF | kubectl apply -f -
apiVersion: batch/v1
kind: CronJob
metadata:
  name: nestgate-backup
  namespace: $NAMESPACE
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          serviceAccountName: nestgate-enterprise
          containers:
          - name: backup
            image: nestgate-backup:latest
            env:
            - name: BACKUP_DESTINATION
              value: "/backup/nestgate"
            - name: S3_BUCKET
              value: "nestgate-enterprise-backups"
            volumeMounts:
            - name: data
              mountPath: /data/nestgate
              readOnly: true
            - name: backup
              mountPath: /backup/nestgate
            command:
            - /bin/sh
            - -c
            - |
              echo "Starting backup at $(date)"
              tar -czf "/backup/nestgate/backup-$(date +%Y%m%d-%H%M%S).tar.gz" /data/nestgate
              echo "Backup completed at $(date)"
          volumes:
          - name: data
            persistentVolumeClaim:
              claimName: nestgate-data
          - name: backup
            persistentVolumeClaim:
              claimName: nestgate-backup
          restartPolicy: OnFailure
EOF
    
    log_success "Backup system configured"
}

# Verify deployment
verify_deployment() {
    log_info "Verifying deployment..."
    
    # Wait for StatefulSet to be ready
    kubectl rollout status statefulset/nestgate-enterprise -n "$NAMESPACE" --timeout=600s
    
    # Check pod status
    local ready_pods
    ready_pods=$(kubectl get pods -n "$NAMESPACE" -l app=nestgate-enterprise --field-selector=status.phase=Running --no-headers | wc -l)
    
    if [ "$ready_pods" -eq "$REPLICAS" ]; then
        log_success "All $REPLICAS NestGate pods are running"
    else
        log_warning "Only $ready_pods out of $REPLICAS pods are running"
    fi
    
    # Check service endpoints
    local endpoints
    endpoints=$(kubectl get endpoints nestgate-enterprise -n "$NAMESPACE" -o jsonpath='{.subsets[*].addresses[*].ip}' | wc -w)
    
    if [ "$endpoints" -gt 0 ]; then
        log_success "Service endpoints are available"
    else
        log_error "No service endpoints available"
    fi
    
    # Get service URL
    local service_ip
    service_ip=$(kubectl get service nestgate-enterprise -n "$NAMESPACE" -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    
    if [ -n "$service_ip" ]; then
        log_success "NestGate is available at: https://$service_ip"
    else
        log_info "LoadBalancer IP is pending. Use 'kubectl get svc -n $NAMESPACE' to check status"
    fi
    
    # Display cluster information
    log_info "Cluster Information:"
    kubectl get pods,svc,pvc -n "$NAMESPACE" -l app=nestgate-enterprise
}

# Cleanup function
cleanup() {
    log_info "Cleaning up temporary resources..."
    # Add cleanup logic if needed
}

# Main deployment function
main() {
    echo "🚀 NestGate Enterprise Deployment"
    echo "=================================="
    echo "Environment: $DEPLOYMENT_ENV"
    echo "Cluster: $CLUSTER_NAME"
    echo "Namespace: $NAMESPACE"
    echo "Replicas: $REPLICAS"
    echo "Storage Size: $STORAGE_SIZE"
    echo "=================================="
    echo
    
    # Set trap for cleanup
    trap cleanup EXIT
    
    # Run deployment steps
    check_prerequisites
    setup_namespace
    deploy_monitoring
    deploy_storage
    create_configuration
    deploy_application
    deploy_ingress
    setup_monitoring
    setup_backup
    verify_deployment
    
    echo
    log_success "🎉 NestGate Enterprise deployment completed successfully!"
    echo
    echo "Next steps:"
    echo "1. Configure DNS to point to the LoadBalancer IP"
    echo "2. Set up SSL certificates with cert-manager"
    echo "3. Configure monitoring alerts"
    echo "4. Set up backup retention policies"
    echo "5. Configure RBAC and user access"
    echo
    echo "Useful commands:"
    echo "  kubectl get all -n $NAMESPACE"
    echo "  kubectl logs -f statefulset/nestgate-enterprise -n $NAMESPACE"
    echo "  kubectl port-forward svc/nestgate-enterprise 8443:443 -n $NAMESPACE"
}

# Run main function
main "$@" 