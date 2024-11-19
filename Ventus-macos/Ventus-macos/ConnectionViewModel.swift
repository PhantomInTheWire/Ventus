//
//  ConnectionViewModel.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct ConnectionCredentials {
    var ipAddress: String
    var folderName: String
    
    var isValid: Bool {
        IPAddressValidator.isValid(ipAddress) && !folderName.isEmpty
    }
}

// MARK: - View Models
class ConnectionViewModel: ObservableObject {
    @Published var credentials = ConnectionCredentials(ipAddress: "", folderName: "")
    @Published var isConnecting = false
    @Published var connectionProgress: CGFloat = 0
    @Published var errorState = ErrorState()
    
    func connect() {
        guard validateInput() else { return }
        
        withAnimation(.spring()) {
            errorState.clear()
            isConnecting = true
            simulateConnectionProgress()
        }
    }
    
    private func validateInput() -> Bool {
        if !IPAddressValidator.isValid(credentials.ipAddress) {
            errorState.show(message: "Please enter a valid IP address")
            return false
        }
        
        if credentials.folderName.isEmpty {
            errorState.show(message: "Please enter a folder name")
            return false
        }
        
        return true
    }
    
    private func simulateConnectionProgress() {
        withAnimation { connectionProgress = 0.3 }
        
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
            withAnimation { self.connectionProgress = 0.7 }
        }
        
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
            withAnimation { self.connectionProgress = 1.0 }
        }
    }
}

// MARK: - Validators
struct IPAddressValidator {
    static func isValid(_ ip: String) -> Bool {
        let parts = ip.split(separator: ".")
        guard parts.count == 4 else { return false }
        
        return parts.allSatisfy { part in
            guard let number = Int(part) else { return false }
            return number >= 0 && number <= 255
        }
    }
}
