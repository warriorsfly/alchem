#!/usr/bin/env python3
"""websocket cmd client for actix/websocket-tcp-chat example."""
import argparse
import asyncio
import json
import signal
import sys

import aiohttp

queue = asyncio.Queue()


async def produce_token():
    for x in range(300000, 302000, 1):
        async with aiohttp.ClientSession() as session:
            resp = await session.post("http://127.0.0.1:3000/api/user/login", json={"name": str(x), "password": "12345678"})
            jwt = await resp.json()
            queue.put(jwt.get("token"))

      
async def consume_token(q):
    while True:
        token = await q.get()
        if token is None:
            # None 是一个停止信号。
            q.task_done()
            break
        else:
            async with aiohttp.ClientSession() as session:
                ws =  await session.ws_connect("http://127.0.0.1:3000/ws", autoclose=False, autoping=False, headers={"Authorization": "Bearer "+token})
                async def dispatch():
                    while True:
                        msg = await ws.receive()
                        if msg.type == aiohttp.WSMsgType.TEXT:
                            print('Text: ', msg.data.strip())
                        elif msg.type == aiohttp.WSMsgType.BINARY:
                            print('Binary: ', msg.data)
                        elif msg.type == aiohttp.WSMsgType.PING:
                            await ws.pong()
                        elif msg.type == aiohttp.WSMsgType.PONG:
                            print('Pong received')
                        else:
                            if msg.type == aiohttp.WSMsgType.CLOSE:
                                await ws.close()
                            elif msg.type == aiohttp.WSMsgType.ERROR:
                                print('Error during receive %s' %
                                        ws.exception())
                            elif msg.type == aiohttp.WSMsgType.CLOSED:
                                pass
                            break

                await dispatch()
            q.task_done()


async def main():
    consumer = asyncio.ensure_future(consume_token(queue))     
    await produce_token()
    await queue.join()
    consumer.cancel()


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()
