-- CreateTable
CREATE TABLE "Account" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "icon" TEXT,
    "currencyId" INTEGER NOT NULL,
    "type" TEXT NOT NULL,
    CONSTRAINT "Account_currencyId_fkey" FOREIGN KEY ("currencyId") REFERENCES "Currency" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Currency" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "code" TEXT NOT NULL,
    "symbol" TEXT,
    "numeric_code" INTEGER NOT NULL,
    "digits" INTEGER NOT NULL DEFAULT 2
);

-- CreateTable
CREATE TABLE "ExchangeRate" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "fromId" INTEGER NOT NULL,
    "toId" INTEGER NOT NULL,
    "rate" REAL NOT NULL,
    CONSTRAINT "ExchangeRate_fromId_fkey" FOREIGN KEY ("fromId") REFERENCES "Currency" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "ExchangeRate_toId_fkey" FOREIGN KEY ("toId") REFERENCES "Currency" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Transaction" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "accountId" TEXT NOT NULL,
    "amount" REAL NOT NULL,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL,
    "notes" TEXT,
    CONSTRAINT "Transaction_accountId_fkey" FOREIGN KEY ("accountId") REFERENCES "Account" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Transfer" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "debitTransactionId" TEXT NOT NULL,
    "creditTransactionId" TEXT NOT NULL,
    CONSTRAINT "Transfer_debitTransactionId_fkey" FOREIGN KEY ("debitTransactionId") REFERENCES "Transaction" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Transfer_debitTransactionId_fkey" FOREIGN KEY ("debitTransactionId") REFERENCES "Transaction" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateIndex
CREATE INDEX "ExchangeRate_fromId_toId_idx" ON "ExchangeRate"("fromId", "toId");

-- CreateIndex
CREATE UNIQUE INDEX "Transfer_debitTransactionId_key" ON "Transfer"("debitTransactionId");

-- CreateIndex
CREATE UNIQUE INDEX "Transfer_creditTransactionId_key" ON "Transfer"("creditTransactionId");
