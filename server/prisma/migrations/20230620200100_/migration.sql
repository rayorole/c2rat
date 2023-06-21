/*
  Warnings:

  - A unique constraint covering the columns `[uid]` on the table `Client` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX `Client_uid_key` ON `Client`(`uid`);
