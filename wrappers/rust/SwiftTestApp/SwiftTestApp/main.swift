//
//  main.swift
//  SwiftTestApp
//
//  Created by Steven H. McCown on 1/13/22.
//

import Foundation

//------------------------------------------------------------
// arrayToJsonString() is a helper function to convert an
// array representation to a Json string into a single
// string representation of the same data.
func arrayToJsonString(data: [String: String]) -> String {
    
    var result: String = ""
    
    do {
        let json = try JSONEncoder().encode(data)

        result = String(data: json, encoding: .utf8)!
    } catch {
        result = ""
        print("Exception : \(error)")
    }

    return result
}

//------------------------------------------------------------
print("----- Start Indy Tests -----")

//------------------------------
// 0. Setup default config.
var e = setRuntimeConfig(config: "{\"crypto_thread_pool_size\": 2}")
print("Setup Runtime Config result = \(e)\n")

//------------------------------
// 1. Create and Open personA Wallet
let personAWalletConfig: [String: String] = ["id": "personAWallet"]
let personAWalletConfigString = arrayToJsonString(data: personAWalletConfig)

let personAWalletCredentials: [String: String] = ["key": "personA_wallet_key"]
let personAWalletCredentialsString = arrayToJsonString(data: personAWalletCredentials)

do {
    // Calls the createWallet() function (through the uniffi interface layer) to the libindy library.
    try createWallet(config: personAWalletConfigString, credentials: personAWalletCredentialsString)
    print("Created personA Wallet")
} catch {
    print("Cannot create personA Wallet.  It probably already exists.")
}
var personAWallet: Int32 = 0
do {
    // Calls the openWallet() function (through the uniffi interface layer) to the libindy library.
    personAWallet = try openWallet(config: personAWalletConfigString, credentials: personAWalletCredentialsString)
    print("Opened personA Wallet\n")
} catch {
    print("Cannot open personA Wallet.  It probably already exists.")
}

//------------------------------
// 2. Create and Open personB Wallet
let personBWalletConfig: [String: String] = ["id": "personBWallet"]
let personBWalletConfigString = arrayToJsonString(data: personBWalletConfig)

let personBWalletCredentials: [String: String] = ["key": "personB_wallet_key"]
let personBWalletCredentialsString = arrayToJsonString(data: personBWalletCredentials)

do {
    // Calls the createWallet() function (through the uniffi interface layer) to the libindy library.
    try createWallet(config: personBWalletConfigString, credentials: personBWalletCredentialsString)
    print("Created personB Wallet")
} catch {
    print("Cannot create personB Wallet.  It probably already exists.")
}
var personBWallet: Int32 = 0
do {
    // Calls the openWallet() function (through the uniffi interface layer) to the libindy library.
    personBWallet = try openWallet(config: personBWalletConfigString, credentials: personBWalletCredentialsString)
    print("Opened personB Wallet\n")
} catch {
    print("Cannot open personB Wallet.  It probably already exists.")
}

//------------------------------
// 3. Create personA Did
// Calls the createAndStoreMyDid() function (through the uniffi interface layer) to the libindy library.
var personADid: StringString = try createAndStoreMyDid(walletHandle: personAWallet, didJson: "{}")
print("personA did = \(personADid.i0) : personA verkey = \(personADid.i1)")

//------------------------------
// 4. Create personB Did
// Calls the createAndStoreMyDid() function (through the uniffi interface layer) to the libindy library.
var personBDid = try createAndStoreMyDid(walletHandle: personBWallet, didJson: "{}")
print("\n")
print("personB did = \(personBDid.i0) : personB verkey = \(personBDid.i1)")
print("\n")

//------------------------------
// 5. List the dids in personA Wallet
do {
    // Calls the listMyDidsWithMetadata() function (through the uniffi interface layer) to the libindy library.
    let personADids = try listMyDidsWithMetadata(walletHandle: personAWallet)
    print("personA Wallet dids = \(personADids)\n")
} catch {
    print("Error listing personA Wallet dids.")
}

//------------------------------
// 6. List the dids in personB Wallet
do{
    // Calls the listMyDidsWithMetadata() function (through the uniffi interface layer) to the libindy library.
    let personBDids = try listMyDidsWithMetadata(walletHandle: personBWallet)
    print("personB Wallet dids = \(personBDids)\n")
} catch {
    print("Error listing personA Wallet dids.")
}

//------------------------------
// 7. Encode Message from personA to personB
let messageA2B = "personA -> personB"
print("Plain Text (A->B): \(messageA2B)")
var encryptedMessageA2B: [UInt8] = []
do {
    // Calls the authCrypt() function (through the uniffi interface layer) to the libindy library.
    encryptedMessageA2B = try authCrypt(walletHandle: personAWallet, senderVk: personADid.i1, recipientVk: personBDid.i1, message: [UInt8](messageA2B.utf8))
    print("Cipher Text (A->B): \(encryptedMessageA2B)")
} catch {
    print("Error in authCrypt()")
}

//------------------------------
// 8. personB decodes message from personA
var decryptedMessageA2B: StringVecU8
do {
    // Calls the authDecrypt() function (through the uniffi interface layer) to the libindy library.
    decryptedMessageA2B = try authDecrypt(walletHandle: personBWallet, recipientVk: personBDid.i1, encryptedMessage: encryptedMessageA2B)
    print("Decrypted (A->B): \(String(decoding: decryptedMessageA2B.i1, as: UTF8.self))\n")
} catch {
    print("Error in authDecrypt()")
}

//------------------------------
// 9. Encode Message from personB to personA
let messageB2A = "personB -> personA"
print("Plain Text (B->A): \(messageB2A)")
var encryptedMessageB2A: [UInt8] = []
do {
    // Calls the authCrypt() function (through the uniffi interface layer) to the libindy library.
    encryptedMessageB2A = try authCrypt(walletHandle: personBWallet, senderVk: personBDid.i1, recipientVk: personADid.i1, message: [UInt8](messageB2A.utf8))
    print("Cipher Text (B->A): \(encryptedMessageB2A)")
} catch {
    print("Error in authCrypt()")
}

//------------------------------
// 10. personA decodes message from personB
var decryptedMessageB2A: StringVecU8
do {
    // Calls the authDecrypt() function (through the uniffi interface layer) to the libindy library.
    decryptedMessageB2A = try authDecrypt(walletHandle: personAWallet, recipientVk: personADid.i1, encryptedMessage: encryptedMessageB2A)
    print("Decrypted (B->A): \(String(decoding: decryptedMessageB2A.i1, as: UTF8.self))\n")
} catch {
    print("Error in authDecrypt()")
}

//------------------------------
// 11. Close and delete personA Wallet
do {
    // Calls the closeWallet() function (through the uniffi interface layer) to the libindy library.
    try closeWallet(walletHandle: personAWallet)
    print("Closed personA Wallet")
} catch {
    print("Cannot close personA Wallet.")
}
do {
    // Calls the deleteWallet() function (through the uniffi interface layer) to the libindy library.
    try deleteWallet(config: personAWalletConfigString, credentials: personAWalletCredentialsString)
    print("Deleted personA Wallet\n")
} catch {
    print("Cannot delete personA Wallet.")
}

//------------------------------
// 12. Close and delete personB Wallet
do {
    // Calls the closeWallet() function (through the uniffi interface layer) to the libindy library.
    try closeWallet(walletHandle: personBWallet)
    print("Closed personB Wallet")
} catch {
    print("Cannot close personA Wallet.")
}
do {
    // Calls the deleteWallet() function (through the uniffi interface layer) to the libindy library.
    try deleteWallet(config: personBWalletConfigString, credentials: personBWalletCredentialsString)
    print("Deleted personB Wallet\n")
} catch {
    print("Cannot delete personA Wallet.")
}

print("----- End Indy Tests -----");

//------------------------------------------------------------
// Additional tests to be created later.
// Anoncreds.demo();
// AnoncredsRevocation.demo();
// Ledger.demo(args[0]);
// Crypto.demo();
// Endorser.demo();
