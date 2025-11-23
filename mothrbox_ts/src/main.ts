import { Context, Hono } from 'hono'
import { readFromWalus, uploadToWalrus } from "./walrus-client.ts";
import { toUint8Array } from "./util/to_int_arr.ts";

const app = new Hono()

app.get('/', (c: Context) => {
  return c.text('Hello Hono!')
})

app.post('/write', async (c) => {
  const body = await c.req.parseBody();
  const data = body['file'] as File;
  const toUint8Arr = await toUint8Array(data);
  const res = await uploadToWalrus(toUint8Arr, data.name);
  return c.json(res)
})

app.get('/read/:blobId', async (c) => {
  const id = c.req.param('blobId') as string;

  const raw = await readFromWalus(id);

  const bytes = raw instanceof Uint8Array ? raw : new Uint8Array(raw);

  return c.newResponse(bytes, 200, {
    'Content-Type': 'application/octet-stream',
  });
});

Deno.serve(app.fetch)
