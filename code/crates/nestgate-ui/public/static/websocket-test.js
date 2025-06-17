// WebSocket Test

// Connect to the WebSocket server
const socket = new WebSocket('ws://localhost:3000/ws');

// Connection opened
socket.addEventListener('open', (event) => {
    console.log('WebSocket connection opened:', event);
    
    // Send a test message
    const testMessage = {
        id: 'connect',
        command: 'subscribe',
        topics: ['SystemHealth', 'ZfsPool', 'Performance']
    };
    
    socket.send(JSON.stringify(testMessage));
    console.log('Test message sent:', testMessage);
});

// Listen for messages
socket.addEventListener('message', (event) => {
    console.log('Message from server:', event.data);
    
    try {
        const data = JSON.parse(event.data);
        console.log('Parsed message:', data);
    } catch (error) {
        console.error('Error parsing message:', error);
    }
});

// Connection closed
socket.addEventListener('close', (event) => {
    console.log('WebSocket connection closed:', event);
});

// Connection error
socket.addEventListener('error', (event) => {
    console.error('WebSocket error:', event);
});

// Add close connection button
function closeConnection() {
    if (socket && socket.readyState === WebSocket.OPEN) {
        socket.close();
        console.log('Connection closed manually');
    }
}

// Add reconnect button
function reconnect() {
    if (socket && socket.readyState !== WebSocket.OPEN) {
        location.reload();
    }
}

// Export functions for use in HTML
window.wsTest = {
    closeConnection,
    reconnect
}; 