//
//  ErrorView.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct ErrorView: View {
    let message: String
    
    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.red.opacity(0.9))
            Text(message)
                .foregroundColor(.red.opacity(0.9))
                .font(.system(size: 13, weight: .medium))
        }
        .padding(.top, 8)
        .transition(.opacity.combined(with: .scale))
    }
}
