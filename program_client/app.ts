import {Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction,} from "@solana/web3.js";
import * as fs from "fs/promises";
import * as path from "path";
import * as os from "os";
import {
    burnPortfolioNftSendAndConfirm,
	CslSplTokenPDAs,
	derivePortfolioMetadataPDA,
	getPortfolioMetadata,
	initializeClient,
	mintPortfolioNftSendAndConfirm,
	transferPortfolioNftSendAndConfirm,
} from "./index";
import {getMinimumBalanceForRentExemptAccount, getMint, TOKEN_PROGRAM_ID,} from "@solana/spl-token";

import * as readline from 'readline';


async function saveKeypairToFile(keypair: Keypair, filename: string) {
    const secretKey = keypair.secretKey.toString(); // Convert the secret key to string
    await fs.writeFile(filename, secretKey); // Save the secret key to a file
}



// Function to prompt for target address
function promptForAddress(): Promise<string> {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    return new Promise((resolve) => {
        rl.question('Please enter the public address on solana devnet of person who needs my portfolio: ', (address) => {
            rl.close();
            resolve(address);
        });
    });
}





async function main(feePayer: Keypair) {
    const args = process.argv.slice(2);
    const connection = new Connection("https://api.devnet.solana.com", {
        commitment: "confirmed",
    });


    const progId = new PublicKey(args[0]!);


    initializeClient(progId, connection);




    /**
     * Create a keypair for the mint
     */
    const mint = Keypair.generate();
    console.info("+==== Mint Address  ====+");
    console.info(mint.publicKey.toBase58());


    /**
	 * Create two wallets
	 */
	const johnDoeWallet = Keypair.generate();
	console.info("+==== John Doe Wallet ====+");
	console.info(johnDoeWallet.publicKey.toBase58());


	const janeDoeWallet = Keypair.generate();
	console.info("+==== Jane Doe Wallet ====+");
	console.info(janeDoeWallet.publicKey.toBase58());


	const rent = await getMinimumBalanceForRentExemptAccount(connection);
	await sendAndConfirmTransaction(
		connection,
		new Transaction()
			.add(
				SystemProgram.createAccount({
					fromPubkey: feePayer.publicKey,
					newAccountPubkey: johnDoeWallet.publicKey,
					space: 0,
					lamports: rent,
					programId: SystemProgram.programId,
				}),
			)
			.add(
				SystemProgram.createAccount({
					fromPubkey: feePayer.publicKey,
					newAccountPubkey: janeDoeWallet.publicKey,
					space: 0,
					lamports: rent,
					programId: SystemProgram.programId,
				}),
			),
		[feePayer, johnDoeWallet, janeDoeWallet],
        );


	/**
	 * Derive the PortfolioNFT Metadata so we can retrieve it later
	 */
	const [portfolioNFTPub] = derivePortfolioMetadataPDA(
		{
			mint: mint.publicKey,
		},
		progId,
	);
	console.info("+==== PortfolioNFT Metadata Address ====+");
	console.info(portfolioNFTPub.toBase58());


    	/**
	 * Derive the John Doe's Associated Token Account, this account will be
	 * holding the minted NFT.
	 */
	const [johnDoeATA] = CslSplTokenPDAs.deriveAccountPDA({
		wallet: johnDoeWallet.publicKey,
		mint: mint.publicKey,
		tokenProgram: TOKEN_PROGRAM_ID,
	});
	console.info("+==== John Doe ATA ====+");
	console.info(johnDoeATA.toBase58());


	/**
	 * Derive the Jane Doe's Associated Token Account, this account will be
	 * holding the minted NFT when John Doe transfer it
	 */
	const [janeDoeATA] = CslSplTokenPDAs.deriveAccountPDA({
		wallet: janeDoeWallet.publicKey,
		mint: mint.publicKey,
		tokenProgram: TOKEN_PROGRAM_ID,
	});
	console.info("+==== Jane Doe ATA ====+");
	console.info(janeDoeATA.toBase58());





    
    
    const targetAddressString = await promptForAddress();
    const targetAddress = new PublicKey(targetAddressString);

    // Derive the Associated Token Account for the target address
    const [targetATA] = CslSplTokenPDAs.deriveAccountPDA({
        wallet: targetAddress,
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
    });



	/**
	 * Mint a new NFT into John's wallet (technically, the Associated Token Account)
	 */

    // Define the JSON object
    const jsonInfo = {
	"user": {
	  "id": 12345,
	  "name": "John Doe",
	  "email": "johndoe@example.com",
	  "roles": ["admin", "editor"]
	},
	"settings": {
	  "theme": "dark",
	  "notifications": true,
	  "language": "en"
	},
	"posts": [
	  {
		"id": 1,
		"title": "Hello World",
		"content": "This is my first post!",
		"tags": ["introduction", "welcome"]
	  },
	  {
		"id": 2,
		"title": "JSON and You",
		"content": "Let's explore JSON files and their uses.",
		"tags": ["json", "tutorial"]
	  }
	]
  };


  // Convert the JSON object to a string
    const jsonInfoString = JSON.stringify(jsonInfo);
    
    // Assuming this is where you define sellerFeeBasisPoints
    let sellerFeeBasisPointsNumber: number = 5000; // your original number value

    // Convert the number to bigint
    let sellerFeeBasisPoints: bigint = BigInt(sellerFeeBasisPointsNumber);

  // Mint a new NFT into John's wallet
  console.info("+==== Minting... ====+");
  await mintPortfolioNftSendAndConfirm({
    wallet: johnDoeWallet.publicKey,
    assocTokenAccount: johnDoeATA,
    name: "Gleb Savelev",
    profession: "Developer",
    skills: "Next JS, Rust, Anchor, React",
    jsonInfo: jsonInfoString,
    signers: {
        feePayer: feePayer,
        funding: feePayer,
        mint: mint,
        owner: johnDoeWallet,
    },
    symbol: "GS",
    description: "Gleb Savelev | Developer Portfolio",
    sellerFeeBasisPoints:sellerFeeBasisPoints,
    image: "https://placehold.co/600x600?text=Gleb%27s+Portfolio",
    collection: "Portfolio",
    properties: ""
});
  console.info("+==== Minted ====+");

	/**
	 * Get the minted token
	 */
	let mintAccount = await getMint(connection, mint.publicKey);
	console.info("+==== Mint ====+");
	console.info(mintAccount);


	/**
	 * Get the PortfolioNFT Metadata
	 */
	let portfolioNFT = await getPortfolioMetadata(portfolioNFTPub);
	console.info("+==== PortfolioNFT Metadata ====+");
	console.info(portfolioNFT);
	console.assert(portfolioNFT!.assocAccount!.toBase58(), johnDoeATA.toBase58());



    /**
	 * Transfer John Doe's NFT to Jane Doe Wallet (technically, the Associated Token Account)
	 */
	console.info("+==== Transferring... ====+");
	await transferPortfolioNftSendAndConfirm({
		wallet: targetAddress,
		assocTokenAccount: targetATA,
		mint: mint.publicKey,
		source: johnDoeATA,
		destination: targetATA,
		signers: {
			feePayer: feePayer,
			funding: feePayer,
			authority: johnDoeWallet,
		},
	});
	console.info("+==== Transferred ====+");





}
  
 







   




fs.readFile(path.join(os.homedir(), ".config/solana/id.json")).then((file) =>
    main(Keypair.fromSecretKey(new Uint8Array(JSON.parse(file.toString())))),
);

