//
//  EnvironmentUtils.swift
//  SwiftTestApp
//
//  Created by Steven H. McCown on 2/3/22.
//

import Foundation

class EnvironmentUtils {
    //--------------------------------------------------
    static func getTestPoolIP(ip: String) -> String {
        let env = ProcessInfo.processInfo.environment
        
        // Check for the TEST_POOL_ID environment variable.
        let testPoolID = env["TEST_POOL_ID"]

        return testPoolID ?? "127.0.0.1"
    }

    //--------------------------------------------------
    static func getIndyHomePath() -> URL {
        
        return FileManager.default.homeDirectoryForCurrentUser.appendingPathComponent("/.indy_client/");
    }

    //--------------------------------------------------
    static func getIndyHomePath(filename: String) -> URL {

        return getIndyHomePath().appendingPathComponent(filename);
    }

    //--------------------------------------------------
    static func getTmpPath() -> URL {
        
        return FileManager.default.temporaryDirectory.appendingPathComponent("/indy/");
    }

    //--------------------------------------------------
    static func getTmpPath(fileName: String) -> URL {
        
        return getTmpPath().appendingPathComponent(fileName);
    }
}

