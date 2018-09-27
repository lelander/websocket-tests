import asyncio
import websockets


async def handle_connection(websocket, path):
    while True:
        msg = await websocket.recv()
        await websocket.send(msg)

start_server = websockets.serve(handle_connection, '0.0.0.0', 3000)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
