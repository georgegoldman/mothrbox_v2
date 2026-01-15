import { Hono } from "hono";
import { calculateWalsForUpload } from "./walrus-client.ts";

const app = new Hono();

app.get("/", (c) => {
  return c.json({ message: "Mothrbox Walrus API" });
});

// Calculate WALs needed for file upload
app.get("/storage-cost", async (c) => {
  const sizeParam = c.req.query("size");

  if (!sizeParam) {
    return c.json(
      { error: "size query parameter is required (in bytes)" },
      400
    );
  }

  const fileSizeBytes = parseInt(sizeParam, 10);

  if (isNaN(fileSizeBytes) || fileSizeBytes <= 0) {
    return c.json({ error: "size must be a positive number" }, 400);
  }

  const costs = await calculateWalsForUpload(fileSizeBytes);

  return c.json({
    fileSizeBytes,
    storageCost: costs.storageCost.toString(),
    writeCost: costs.writeCost.toString(),
    totalCost: costs.totalCost.toString(),
    totalCostInWals: costs.totalCostInWals,
  });
});

Deno.serve({ port: 3000 }, app.fetch);
