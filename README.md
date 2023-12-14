# riseinProject
This is my Solana bootcamp project 

Programm which mints my Portfolio as NFT, afterwars you can send it to any public Address.

NFT has also some basic json data file, which will be later filled with content from my Sanity CMS so i can show up sections from my portfolio website for users who own this NFT.

just run the program via front end
Program ID: 8RQ5EJsEarrovdJbwax6bNN4SspbjbRmwcuACCHJzzhD
 

yarn install

yarn add @solana/spl-token


also make sure that Solana's Install Tool installed and you generated your key:
sh -c "$(curl -sSfL https://release.solana.com/v1.17.10/install)"


cd /program_client

npx ts-node app.ts 8RQ5EJsEarrovdJbwax6bNN4SspbjbRmwcuACCHJzzhD
