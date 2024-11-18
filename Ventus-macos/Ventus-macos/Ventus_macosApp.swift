//
//  Ventus_macosApp.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 18/11/24.
//

import SwiftUI

@main
struct Ventus_macosApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
        .windowStyle(.hiddenTitleBar)  // Modern macOS style
        .windowResizability(.contentSize)
    }
}
