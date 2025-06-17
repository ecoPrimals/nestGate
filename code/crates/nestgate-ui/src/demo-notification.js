// Demo script for triggering notifications
// Run this in the browser console when visiting http://localhost:3000

const sendMockNotification = (type = 'info', message = 'Test notification', category = 'System') => {
  const types = ['info', 'warning', 'error', 'success'];
  if (!types.includes(type)) {
    console.error(`Type must be one of: ${types.join(', ')}`);
    return;
  }
  
  const notification = {
    id: `demo-notify-${Date.now()}`,
    type: type,
    title: `${type.charAt(0).toUpperCase()}${type.slice(1)} Notification`,
    message: message,
    read: false,
    timestamp: Date.now(),
    category: category,
    source: 'Demo Script'
  };

  const event = new CustomEvent('mockWebSocketMessage', {
    detail: {
      type: 'notification',
      timestamp: Date.now(),
      data: notification
    }
  });
  
  document.dispatchEvent(event);
  console.log(`📨 Sent "${type}" notification: "${message}"`);
};

const sendActionableNotification = (message = 'Action required', category = 'System') => {
  const notification = {
    id: `demo-action-${Date.now()}`,
    type: 'warning',
    title: 'Action Required',
    message: message,
    read: false,
    timestamp: Date.now(),
    category: category,
    source: 'Demo Script',
    actionable: true,
    action: {
      label: 'Take Action'
    }
  };

  const event = new CustomEvent('mockWebSocketMessage', {
    detail: {
      type: 'notification',
      timestamp: Date.now(),
      data: notification
    }
  });
  
  document.dispatchEvent(event);
  console.log(`🔔 Sent actionable notification: "${message}"`);
};

const sendMultipleNotifications = (count = 5) => {
  const types = ['info', 'warning', 'error', 'success'];
  const messages = [
    'Disk temperature warning on disk2',
    'Snapshot cleanup complete',
    'Pool capacity approaching 80%',
    'System update available',
    'Backup task completed',
    'Network interface eth0 down',
    'ZFS error detected',
    'CPU usage spike detected',
    'Memory usage above threshold',
    'Scrub completed on pool0'
  ];
  
  const categories = ['System', 'Storage', 'Network', 'Backup', 'Security', 'Updates'];
  
  for (let i = 0; i < count; i++) {
    const type = types[Math.floor(Math.random() * types.length)];
    const message = messages[Math.floor(Math.random() * messages.length)];
    const category = categories[Math.floor(Math.random() * categories.length)];
    
    setTimeout(() => {
      sendMockNotification(type, message, category);
    }, i * 800); // Stagger notifications
  }
  
  console.log(`🔄 Sending ${count} random notifications...`);
};

// Help message
console.log(`
✨ NotificationCenter Demo Script ✨
Use these functions to test the notification center:

- sendMockNotification(type, message, category)
  Example: sendMockNotification('warning', 'Disk space low', 'Storage')
  Types: 'info', 'warning', 'error', 'success'

- sendActionableNotification(message, category)
  Example: sendActionableNotification('Pool needs repair', 'Storage')

- sendMultipleNotifications(count)
  Example: sendMultipleNotifications(10)
`); 