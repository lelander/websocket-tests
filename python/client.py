import asyncio
import websockets
import time

async def connect():
    async with websockets.connect('ws://localhost:3000') as websocket:
        count = 10000
        duration = 0

        for _ in range(count):
            start = time.time() * 1000000
            await websocket.send('1538011457915')
            await websocket.recv()
            end = time.time() * 1000000
            duration += end - start

        print('Round-trip time: {}us'.format(duration/count))

asyncio.get_event_loop().run_until_complete(connect())
