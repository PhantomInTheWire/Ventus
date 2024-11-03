import unittest
from unittest.mock import AsyncMock, patch
from file_sync_pb2 import SyncResponse, Notification
from client import FileSyncClient  # Replace with your actual import path

class TestFileSyncClient(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.client_id = 'client1'
        self.client = FileSyncClient(self.client_id)
        await self.client.channel.__aenter__()  # Initialize gRPC channel in async test

    async def asyncTearDown(self):
        await self.client.channel.__aexit__(None, None, None)

    @patch('client.FileSyncClient.stub')
    async def test_start_sync(self, mock_stub):
        mock_stub.StartSync = AsyncMock(return_value=SyncResponse(status="Sync started"))
        response = await self.client.start_sync()
        self.assertEqual(response.status, "Sync started")

    @patch('client.FileSyncClient.stub')
    async def test_notify_sync(self, mock_stub):
        notification = Notification(message="Sync complete", origin_client_id="client2")
        response = await self.client.NotifySync(notification)
        self.assertEqual(response.status, "Notification received")

if __name__ == '__main__':
    unittest.main()

