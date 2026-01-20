import { Hono } from "hono";
import { cors } from "hono/cors";
import { calculateWalsForUpload } from "./walrus-client.ts";

const app = new Hono();

// Enable CORS for mothrbox.vercel.app only
app.use("*", cors({ origin: "https://mothrbox.vercel.app" }));

app.get("/", (c) => {
  return c.json({ message: "Mothrbox Walrus API" });
});

// Calculate WALs needed for file upload
app.get("/storage-cost", async (c) => {
  const sizeParam = c.req.query("fileSize");
  const epochsParam = c.req.query("epochs");

  if (!sizeParam) {
    return c.json(
      { error: "fileSize query parameter is required (in bytes)" },
      400,
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
    totalCost: costs.totalCost.toString(),
    totalCostInSui: costs.totalCostInSui,
    totalCostInUsd: costs.totalCostInUsd,
  });
});

const port = Number(Deno.env.get("PORT") ?? 3000);
Deno.serve({ port }, (req) => app.fetch(req));
