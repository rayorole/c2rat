import express, { Express, Request, Response } from "express";
import prisma from "./lib/mysql";

const app: Express = express();
const port = 3000;

app.use(express.json());

app.get("/", (req: Request, res: Response) => {
  res.status(200).json({
    system: "online",
  });
});

app.post("/register", (req: Request, res: Response) => {
  prisma.client
    .create({
      data: {
        uid: req.body.uid,
        username: req.body.username,
        ip: req.body.ip,
        lang: req.body.lang,
        os: req.body.os,
        hostname: req.body.hostname,
        fullname: req.body.fullname,
      },
    })
    .then((data) => {
      res.status(200).json({
        status: "success",
        message: `client ${data.uid} registered`,
      });
    })
    .catch((err) => {
      // If duplicate entry, send 409
      if (err.code === "P2002") {
        res.status(409).json({
          status: "error",
          message: "client already registered",
        });
      } else if (err.message.includes("Can't reach database server")) {
        res.status(500).json({
          status: "error",
          message: "database server unreachable",
        });
      } else {
        res.status(500).json({
          status: "error",
          message: err,
        });
      }
    });
});

app.listen(port, () => {
  console.log(`[*] Server running at http://localhost:${port}`);
});
