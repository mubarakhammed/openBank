-- Add company and title fields to developers table
-- Migration: Add company and title to developers
-- Date: 2025-09-29

ALTER TABLE developers 
ADD COLUMN company VARCHAR(100),
ADD COLUMN title VARCHAR(100);