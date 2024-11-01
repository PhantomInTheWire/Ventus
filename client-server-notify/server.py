import asyncio
import grpc
from concurrent import futures
from file_sync_pb2 import SyncResponse, Notification
import file_sync_pb2_grpc

class FileSyncService(file_sync_pb2_grpc.FileSyncServiceServicer):
    def __init__(self):
        self.connected_clients = {}  # Maps client_id to gRPC context

    async def NotifySync(self, client_id, message):
        client_stub = self.connected_clients.get(client_id)
        if client_stub:
            try:
                await client_stub.NotifySync(Notification(message=message, origin_client_id=client_id))
            except grpc.RpcError as e:
                print(f"Notification error for {client_id}: {e}")
                self.connected_clients.pop(client_id, None)  # Remove disconnected client

    async def StartSync(self, request, context):
        self.connected_clients[request.client_id] = context
        print(f"Client {request.client_id} started sync.")
        return SyncResponse(status="Sync started")

    async def SyncComplete(self, request, context):
        print(f"Sync completed by client {request.client_id}")
        message = f"Client {request.client_id} completed synchronization."

        # Notify all clients except the originating client
        tasks = [
            self.NotifySync(client_id, message)
            for client_id in self.connected_clients
            if client_id != request.client_id
        ]
        await asyncio.gather(*tasks)
        return SyncResponse(status="Sync notifications sent")

    def serve(self):
        server = grpc.aio.server()
        file_sync_pb2_grpc.add_FileSyncServiceServicer_to_server(self, server)
        server.add_insecure_port('[::]:50051')
        asyncio.get_event_loop().run_until_complete(server.start())
        print("Server running on port 50051")
        asyncio.get_event_loop().run_forever()

if __name__ == "__main__":
    service = FileSyncService()
    service.serve()

