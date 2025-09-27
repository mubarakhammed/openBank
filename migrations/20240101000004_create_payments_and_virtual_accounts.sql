-- Create payment status enum
CREATE TYPE payment_status AS ENUM ('pending', 'processing', 'completed', 'failed', 'cancelled', 'refunded');

-- Create payment method enum
CREATE TYPE payment_method AS ENUM ('bank_transfer', 'card', 'wallet', 'crypto');

-- Create payments table
CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_account_id UUID NOT NULL REFERENCES accounts(id),
    to_account_id UUID REFERENCES accounts(id),
    amount BIGINT NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) DEFAULT 'USD',
    payment_method payment_method NOT NULL,
    status payment_status DEFAULT 'pending',
    reference VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    recipient_info JSONB,
    metadata JSONB,
    external_reference VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create virtual account status enum
CREATE TYPE virtual_account_status AS ENUM ('active', 'inactive', 'suspended', 'closed');

-- Create virtual_accounts table
CREATE TABLE IF NOT EXISTS virtual_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    parent_account_id UUID NOT NULL REFERENCES accounts(id),
    account_number VARCHAR(20) UNIQUE NOT NULL,
    account_name VARCHAR(255) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',
    status virtual_account_status DEFAULT 'active',
    purpose TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_payments_from_account ON payments(from_account_id);
CREATE INDEX IF NOT EXISTS idx_payments_to_account ON payments(to_account_id);
CREATE INDEX IF NOT EXISTS idx_payments_reference ON payments(reference);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_virtual_accounts_user_id ON virtual_accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_virtual_accounts_parent_account ON virtual_accounts(parent_account_id);
CREATE INDEX IF NOT EXISTS idx_virtual_accounts_account_number ON virtual_accounts(account_number);