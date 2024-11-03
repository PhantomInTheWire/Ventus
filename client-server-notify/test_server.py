import unittest
from unittest.mock import AsyncMock, patch
from file_sync_pb2 import ClientRequest, SyncResponse, Notification
from server import FileSyncService  # Replace with your actual import path

class TestFileSyncService(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.service = FileSyncService()
    
    @patch('server.FileSyncService.NotifySync', new_callable=AsyncMock)
    async def test_sync_complete(self, mock_notify_sync):
        # Add mock clients
        self.service.connected_clients['client1'] = AsyncMock()
        self.service.connected_clients['client2'] = AsyncMock()

        # Run SyncComplete for client1
        request = ClientRequest(client_id='client1')
        response = await self.service.SyncComplete(request, context=None)

        # Verify response
        self.assertEqual(response.status, "Sync notifications sent")

        # Ensure NotifySync was called only for client2
        mock_notify_sync.assert_called_once_with('client2', 'Client client1 completed synchronization.')

    async def test_start_sync(self):
        request = ClientRequest(client_id='client1')
        response = await self.service.StartSync(request, context=None)
        
        # Verify client is registered
        self.assertIn('client1', self.service.connected_clients)
        self.assertEqual(response.status, "Sync started")

if __name__ == '__main__':
    unittest.main()

