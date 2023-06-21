// Export a Prisma client I can use to connect to the database
import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

export default prisma;
