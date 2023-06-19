import express, { Express, Request, Response } from "express";

const app: Express = express();
const port = 3000;

app.use(express.json());

app.get("/", (req: Request, res: Response) => {
  res.status(200).json({
    system: "online",
  });
});

app.post("/register", (req: Request, res: Response) => {
  console.log(req.body);
  res.status(200).json({
    message: req.body.uid,
  });
});

app.listen(port, () => {
  console.log(`[*] Server running at http://localhost:${port}`);
});
