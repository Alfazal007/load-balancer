import express, { Request, Response } from "express";

import { configDotenv } from "dotenv";

configDotenv();

const app = express();

app.use(express.json());

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
