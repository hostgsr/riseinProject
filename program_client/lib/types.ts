// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

export interface PortfolioMetadata {
  name: string;
  symbol: string;
  description: string;
  sellerFeeBasisPoints: bigint;
  image: string;
  collection: string;
  properties: string;
  profession: string;
  skills: string;
  jsonInfo: string;
  mint: PublicKey;
  assocAccount: PublicKey | undefined;
}

export const decodePortfolioMetadata = (decoded: Decoded): PortfolioMetadata => ({
    name: decoded["name"] as string,
    symbol: decoded["symbol"] as string,
    description: decoded["description"] as string,
    sellerFeeBasisPoints: decoded["seller_fee_basis_points"] as bigint,
    image: decoded["image"] as string,
    collection: decoded["collection"] as string,
    properties: decoded["properties"] as string,
    profession: decoded["profession"] as string,
    skills: decoded["skills"] as string,
    jsonInfo: decoded["json_info"] as string,
    mint: new PublicKey(decoded["mint"] as Uint8Array),
    assocAccount: decoded["assoc_account"] ? new PublicKey(decoded["assoc_account"]) : undefined,
});

export const PortfolioMetadataSchema: Schema =  {
    struct: {
        name: "string",
        symbol: "string",
        description: "string",
        seller_fee_basis_points: "u64",
        image: "string",
        collection: "string",
        properties: "string",
        profession: "string",
        skills: "string",
        json_info: "string",
        mint: { array: { type: "u8", len: 32 } },
        assoc_account: { option: { array: { type: "u8", len: 32 } } },
    }
};

export module CslSplTokenTypes {
    /// Mint data.
    export interface Mint {
      mintAuthority: PublicKey;
      supply: bigint;
      decimals: number;
      isInitialized: boolean;
      freezeAuthority: PublicKey;
    }
    
    export const decodeMint = (decoded: Decoded): Mint => ({
        mintAuthority: new PublicKey(decoded["mint_authority"] as Uint8Array),
        supply: decoded["supply"] as bigint,
        decimals: decoded["decimals"] as number,
        isInitialized: decoded["is_initialized"] as boolean,
        freezeAuthority: new PublicKey(decoded["freeze_authority"] as Uint8Array),
    });
    
    export const MintSchema: Schema =  {
        struct: {
            mint_authority: { array: { type: "u8", len: 32 } },
            supply: "u64",
            decimals: "u8",
            is_initialized: "bool",
            freeze_authority: { array: { type: "u8", len: 32 } },
        }
    };
    
    /// Account data
    export interface Account {
      mint: PublicKey;
      owner: PublicKey;
      amount: bigint;
      delegate: PublicKey;
      state: number;
      isNative: bigint;
      delegatedAmount: bigint;
      closeAuthority: PublicKey;
    }
    
    export const decodeAccount = (decoded: Decoded): Account => ({
        mint: new PublicKey(decoded["mint"] as Uint8Array),
        owner: new PublicKey(decoded["owner"] as Uint8Array),
        amount: decoded["amount"] as bigint,
        delegate: new PublicKey(decoded["delegate"] as Uint8Array),
        state: decoded["state"] as number,
        isNative: decoded["is_native"] as bigint,
        delegatedAmount: decoded["delegated_amount"] as bigint,
        closeAuthority: new PublicKey(decoded["close_authority"] as Uint8Array),
    });
    
    export const AccountSchema: Schema =  {
        struct: {
            mint: { array: { type: "u8", len: 32 } },
            owner: { array: { type: "u8", len: 32 } },
            amount: "u64",
            delegate: { array: { type: "u8", len: 32 } },
            state: "u8",
            is_native: "u64",
            delegated_amount: "u64",
            close_authority: { array: { type: "u8", len: 32 } },
        }
    };
    
}



