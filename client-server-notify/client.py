import grpc
import asyncio
from file_sync_pb2 import ClientRequest, Notification, SyncResponse
import file_sync_pb2_grpc

class FileSyncClient:
    def __init__(self, client_id):
        self.client_id = client_id
        self.channel = grpc.aio.insecure_channel('localhost:50051')
        self.stub = file_sync_pb2_grpc.FileSyncServiceStub(self.channel)

    async def start_sync(self):
        request = ClientRequest(client_id=self.client_id)
        response = await self.stub.StartSync(request)
        print(f"StartSync Response: {response.status}")

    async def sync_complete(self):
        request = ClientRequest(client_id=self.client_id)
        response = await self.stub.SyncComplete(request)
        print(f"SyncComplete Response: {response.status}")

    async def NotifySync(self, notification):
        print(f"Received notification: {notification.message} from {notification.origin_client_id}")
        return SyncResponse(status="Notification received")

    async def run(self):
        await self.channel.subscribe(self.NotifySync)  # Subscribe to notifications
        await self.start_sync()
        
        # Simulate work then mark sync as complete
        await asyncio.sleep(2)
        await self.sync_complete()

        # Keep client running to receive notifications
        await asyncio.Event().wait()

if __name__ == "__main__":
    client_id = "client1"  # Unique identifier for each client
    client = FileSyncClient(client_id)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(client.run())

