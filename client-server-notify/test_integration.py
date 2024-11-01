import pytest
import asyncio
from server import FileSyncService
from client import FileSyncClient

@pytest.mark.asyncio
async def test_integration():
    # Start the server
    server_service = FileSyncService()
    server_task = asyncio.create_task(server_service.serve())

    # Simulate two clients
    client1 = FileSyncClient('client1')
    client2 = FileSyncClient('client2')
    
    await asyncio.gather(
        client1.start_sync(),
        client2.start_sync()
    )
    
    await asyncio.sleep(1)  # Allow time for synchronization process
    
    # Client 1 completes sync
    await client1.sync_complete()

    await asyncio.sleep(1)  # Allow time for notifications to be received

    # Verify both clients received notifications
    # (For example, check if expected print statements occurred in stdout or logs)

    # Cleanup
    server_task.cancel()  # Stop the server
    await asyncio.gather(client1.channel.close(), client2.channel.close())


