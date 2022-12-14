/*
  Warnings:

  - You are about to drop the column `type` on the `Account` table. All the data in the column will be lost.
  - Added the required column `accountType` to the `Account` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Account" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "icon" TEXT,
    "currencyId" INTEGER NOT NULL,
    "accountType" TEXT NOT NULL,
    CONSTRAINT "Account_currencyId_fkey" FOREIGN KEY ("currencyId") REFERENCES "Currency" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Account" ("currencyId", "icon", "id", "name") SELECT "currencyId", "icon", "id", "name" FROM "Account";
DROP TABLE "Account";
ALTER TABLE "new_Account" RENAME TO "Account";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
