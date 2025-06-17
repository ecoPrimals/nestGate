const http = require('http');
const fs = require('fs');
const path = require('path');

const port = 3000;
const srcPath = path.join(__dirname, 'src');

// Simple server
const server = http.createServer((req, res) => {
  console.log(`Request: ${req.url}`);
  
  // Default to index.html
  let filePath = req.url === '/' ? '/index.html' : req.url;
  
  // Map CSS files
  if (filePath.endsWith('.css')) {
    const cssFile = path.join(__dirname, filePath);
    if (fs.existsSync(cssFile)) {
      res.writeHead(200, { 'Content-Type': 'text/css' });
      fs.createReadStream(cssFile).pipe(res);
      return;
    }
  }
  
  // Map JavaScript files
  if (filePath.endsWith('.js')) {
    const jsFile = path.join(__dirname, filePath);
    if (fs.existsSync(jsFile)) {
      res.writeHead(200, { 'Content-Type': 'application/javascript' });
      fs.createReadStream(jsFile).pipe(res);
      return;
    }
  }
  
  // Serve HTML
  if (filePath === '/index.html') {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(`
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8">
        <title>NestGate ZFS Dashboard Demo</title>
        <link rel="stylesheet" href="/src/demo.css">
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/antd@5.0.0/dist/reset.css">
        <style>
          body {
            margin: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
              'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
              sans-serif;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
          }
          
          .demo-layout {
            min-height: 100vh;
          }
          
          .demo-header {
            display: flex;
            align-items: center;
            background: #1890ff;
            padding: 0 24px;
          }
          
          .demo-content {
            padding: 24px;
          }
          
          .demo-tabs {
            background: white;
            padding: 24px;
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
          }
          
          /* Responsive adjustments */
          @media (max-width: 768px) {
            .demo-content {
              padding: 12px;
            }
            
            .demo-tabs {
              padding: 12px;
            }
          }
        </style>
      </head>
      <body>
        <div id="root">
          <h1 style="text-align: center; margin-top: 40px;">NestGate ZFS Dashboard Demo</h1>
          
          <div style="max-width: 800px; margin: 0 auto; padding: 20px;">
            <h2>System Metrics Component</h2>
            <p>The System Metrics component provides a comprehensive view of your NAS system including:</p>
            <ul>
              <li>Storage Pool Usage</li>
              <li>System Health Statistics</li>
              <li>Service Status (NFS, SMB, FTP)</li>
              <li>Performance Metrics</li>
              <li>Network Interface Status</li>
            </ul>
            
            <h2>Performance Optimizer Component</h2>
            <p>The Performance Optimizer component allows you to:</p>
            <ul>
              <li>View Storage Pools and Datasets</li>
              <li>Apply Performance Optimizations</li>
              <li>Monitor Optimization Impact</li>
              <li>Schedule Optimization Tasks</li>
            </ul>
            
            <div style="margin-top: 40px; text-align: center; color: #666;">
              <p>Note: This is a static demo with sample data. In a real application, these components would connect to the Tauri backend for live data.</p>
            </div>
          </div>
        </div>
      </body>
      </html>
    `);
    return;
  }
  
  res.writeHead(404);
  res.end('Not found');
});

server.listen(port, () => {
  console.log(`Demo server running at http://localhost:${port}`);
}); 