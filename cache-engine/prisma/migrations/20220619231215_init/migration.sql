-- CreateTable
CREATE TABLE "analyze_data" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "block_hash" VARCHAR(66) NOT NULL,
    "tx_hash" VARCHAR(66) NOT NULL,
    "section" VARCHAR NOT NULL,
    "method" VARCHAR NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "analyze_data_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "blocks" (
    "number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL,

    CONSTRAINT "blocks_pkey" PRIMARY KEY ("number")
);

-- CreateTable
CREATE TABLE "assets" (
    "id" INTEGER NOT NULL,
    "name" TEXT,
    "symbol" TEXT,
    "block_number" INTEGER,
    "hash" VARCHAR(66),
    "owner" VARCHAR(48),
    "decimals" INTEGER,
    "is_sufficient" BOOLEAN NOT NULL DEFAULT false,
    "is_frozen" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP(6),
    "updated_at" TIMESTAMP(6),

    CONSTRAINT "assets_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_transactions" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "sender" VARCHAR(48) NOT NULL,
    "recipient" VARCHAR(48) NOT NULL,
    "amount" INTEGER NOT NULL,
    "asset_id" INTEGER NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL,

    CONSTRAINT "asset_transactions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "description" VARCHAR(128) NOT NULL,
    "category" INTEGER NOT NULL,
    "auditors" INTEGER NOT NULL,
    "block_start" INTEGER NOT NULL,
    "block_end" INTEGER NOT NULL,
    "rewards_token" INTEGER NOT NULL,
    "rewards_amount" INTEGER NOT NULL,
    "rewards_oracle" INTEGER NOT NULL,
    "rewards_auditors" INTEGER NOT NULL,
    "slashing_auditors" INTEGER NOT NULL,
    "max_errors_auditor" INTEGER NOT NULL,
    "fields" VARCHAR(8192) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_approval_requests" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "info" VARCHAR(8192) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "date_approved" TIMESTAMP(6),
    "date_refused" TIMESTAMP(6),
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_approval_requests_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_approval_requests_auditors" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "auditor" VARCHAR(64) NOT NULL,
    "max_days" INTEGER NOT NULL,
    "approval_request_id" INTEGER NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_approval_requests_auditors_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_approval_requests_auditors_votes" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "vote" VARCHAR(1) NOT NULL,
    "other_info" VARCHAR(66) NOT NULL,
    "rewards" TIMESTAMP(6) NOT NULL,
    "approval_request_id" INTEGER NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_approval_requests_auditors_votes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_auditors" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "description" VARCHAR(128) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "account" VARCHAR(48) NOT NULL,
    "categories" VARCHAR(128) NOT NULL,
    "area" VARCHAR(64) NOT NULL,
    "other_info" VARCHAR(66) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_auditors_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_categories" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "description" VARCHAR(128) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_categories_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_oracles" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "description" VARCHAR(128) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "account" VARCHAR(48) NOT NULL,
    "other_info" VARCHAR(66) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_oracles_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "impact_actions_proxies" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "signer" VARCHAR(64) NOT NULL,
    "account" VARCHAR(48) NOT NULL,
    "date" TIMESTAMP(6) NOT NULL,
    "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "impact_actions_proxies_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "transactions" (
    "id" SERIAL NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "sender" VARCHAR(64) NOT NULL,
    "recipient" VARCHAR(64) NOT NULL,
    "amount" DECIMAL(32,0) NOT NULL,
    "gas_fees" DECIMAL(32,0) NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP(6) NOT NULL,

    CONSTRAINT "transactions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_projects" (
    "id" INTEGER NOT NULL,
    "block_number" INTEGER NOT NULL,
    "hash" VARCHAR(66) NOT NULL,
    "asset_id" INTEGER,
    "originator" VARCHAR(64) NOT NULL,
    "name" VARCHAR,
    "description" TEXT,
    "registry_name" VARCHAR,
    "registry_id" TEXT,
    "registry_summary" TEXT,
    "approved" BOOLEAN DEFAULT false,
    "total_supply" INTEGER,
    "minted" INTEGER,
    "retired" INTEGER,
    "unit_price" REAL,
    "created_at" TIMESTAMP(6) NOT NULL,
    "updated_at" TIMESTAMP(6),

    CONSTRAINT "vcu_projects_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_batches" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "name" VARCHAR,
    "uuid" TEXT,
    "issuance_year" INTEGER,
    "start_date" TEXT,
    "end_date" TEXT,
    "total_supply" INTEGER,
    "minted" INTEGER,
    "retired" INTEGER,

    CONSTRAINT "vcu_project_batches_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_documents" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "url" TEXT,

    CONSTRAINT "vcu_project_documents_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_images" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "url" TEXT,

    CONSTRAINT "vcu_project_images_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_locations" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "latitude" REAL,
    "longitude" REAL,

    CONSTRAINT "vcu_project_locations_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_royalties" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "account" VARCHAR(64) NOT NULL,
    "fee_percent" REAL,

    CONSTRAINT "vcu_project_royalties_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_sdgs" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "type" TEXT,
    "description" TEXT,
    "references" TEXT,

    CONSTRAINT "vcu_project_sdgs_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "vcu_project_videos" (
    "id" SERIAL NOT NULL,
    "project_id" INTEGER NOT NULL,
    "url" TEXT,

    CONSTRAINT "vcu_project_videos_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "blocks_hash_key" ON "blocks"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "assets_hash_key" ON "assets"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "asset_transactions_hash_key" ON "asset_transactions"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_hash_key" ON "impact_actions"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_approval_requests_hash_key" ON "impact_actions_approval_requests"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_approval_requests_auditors_hash_key" ON "impact_actions_approval_requests_auditors"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_approval_requests_auditors_votes_hash_key" ON "impact_actions_approval_requests_auditors_votes"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_auditors_hash_key" ON "impact_actions_auditors"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_categories_hash_key" ON "impact_actions_categories"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_oracles_hash_key" ON "impact_actions_oracles"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "impact_actions_proxies_hash_key" ON "impact_actions_proxies"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "transactions_hash_key" ON "transactions"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "vcu_projects_hash_key" ON "vcu_projects"("hash");

-- CreateIndex
CREATE UNIQUE INDEX "vcu_projects_asset_id_key" ON "vcu_projects"("asset_id");

-- AddForeignKey
ALTER TABLE "vcu_projects" ADD CONSTRAINT "vcu_projects_asset_id_fkey" FOREIGN KEY ("asset_id") REFERENCES "assets"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_batches" ADD CONSTRAINT "vcu_project_batches_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_documents" ADD CONSTRAINT "vcu_project_documents_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_images" ADD CONSTRAINT "vcu_project_images_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_locations" ADD CONSTRAINT "vcu_project_locations_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_royalties" ADD CONSTRAINT "vcu_project_royalties_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_sdgs" ADD CONSTRAINT "vcu_project_sdgs_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "vcu_project_videos" ADD CONSTRAINT "vcu_project_videos_project_id_fkey" FOREIGN KEY ("project_id") REFERENCES "vcu_projects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
