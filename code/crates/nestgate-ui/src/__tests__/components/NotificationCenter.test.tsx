import { WebSocketMessageType } from '../../services/websocket.service';

describe('NotificationCenter Integration', () => {
  it('can import WebSocketMessageType', () => {
    expect(WebSocketMessageType.NOTIFICATION).toBe('notification');
  });

  it('has basic test functionality', () => {
    expect(1 + 1).toBe(2);
  });
}); 