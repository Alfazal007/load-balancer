import { NextFunction, Request, Response } from "express";
import client from "prom-client";

const requestCounter = new client.Counter({
	name: 'http_requests_total',
	help: 'Total number of HTTP requests',
	labelNames: ['method', 'route', 'status_code']
});

export const promMiddleware = (req: Request, res: Response, next: NextFunction) => {
	res.on("finish", () => {
		requestCounter.inc({
			method: req.method,
			status_code: res.statusCode,
			route: req.path
		})
	});
	next();
}
