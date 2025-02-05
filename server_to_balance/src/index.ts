import express, { Request, Response } from "express";

import { configDotenv } from "dotenv";
import { promMiddleware } from "./middleware";
import client from "prom-client";

configDotenv();

const app = express();

app.use(express.json());
app.use(promMiddleware);

app.get("/hello", (_req: Request, res: Response) => {
	res.status(200).json({
		message: "hello"
	});
	return;
});

app.post("/sendData", (req: Request, res: Response) => {
	res.status(200).json(req.body);
	return;
});

app.get("/health", (_req, res) => {
	res.status(200).json({
		message: "working"
	});
	return;
});

app.listen(process.env.PORT, () => {
	console.log(`App listening on port ${process.env.PORT}`)
});

app.get("/metrics", async (_req, res) => {
	const metrics = await client.register.metrics();
	res.set('Content-Type', client.register.contentType);
	res.end(metrics);
});
