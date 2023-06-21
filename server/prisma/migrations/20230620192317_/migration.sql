-- CreateTable
CREATE TABLE `Client` (
    `id` INTEGER NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(191) NOT NULL,
    `ip` VARCHAR(191) NULL,
    `lang` VARCHAR(191) NULL,
    `fullname` VARCHAR(191) NULL,
    `uid` VARCHAR(191) NULL,
    `hostname` VARCHAR(191) NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
