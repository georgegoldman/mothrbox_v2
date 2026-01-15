import { Hono } from "hono";
import { calculateWalsForUpload } from "./walrus-client.ts";

const app = new Hono();

app.get("/", (c) => {
  return c.json({ message: "Mothrbox Walrus API" });
});

// Calculate WALs needed for file upload
app.get("/storage-cost", async (c) => {
  const sizeParam = c.req.query("size");
  const epochsParam = c.req.query("epochs");

  if (!sizeParam) {
    return c.json(
      { error: "size query parameter is required (in bytes)" },
      400
    );
  }

  const fileSizeBytes = Number(sizeParam);
  const epochs = epochsParam ? Number(epochsParam) : 3;

  if (isNaN(fileSizeBytes) || fileSizeBytes <= 0) {
    return c.json({ error: "size must be a positive number" }, 400);
  }

  if (isNaN(epochs) || epochs <= 0 || !Number.isInteger(epochs)) {
    return c.json({ error: "epochs must be a positive integer" }, 400);
  }

  const costs = await calculateWalsForUpload(fileSizeBytes, epochs);

  return c.json({
    fileSizeBytes,
    epochs,
    storageCost: costs.storageCost.toString(),
    writeCost: costs.writeCost.toString(),
    totalCost: costs.totalCost.toString(),
    totalCostInWals: costs.totalCostInWals,
  });
});

const port = Deno.env.get("PORT") ?? 3000;
Deno.serve({ port }, app.fetch);
