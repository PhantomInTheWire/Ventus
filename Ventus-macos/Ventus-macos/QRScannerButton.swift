//
//  QRScannerButton.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct QRScannerButton: View {
    @Binding var showingScanner: Bool
    
    var body: some View {
        Button(action: { showingScanner = true }) {
            Image(systemName: "qrcode.viewfinder")
                .foregroundColor(.white)
                .font(.system(size: 20))
                .frame(width: 40, height: 40)
                .background(
                    LinearGradient(
                        gradient: Gradient(colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)]),
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
                .clipShape(Circle())
        }
        .buttonStyle(.plain)
        .padding(.trailing, 20)
    }
}
