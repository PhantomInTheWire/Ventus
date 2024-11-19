//
//  StatBox.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct StatBox: View {
    let icon: String
    let iconColor: Color
    let title: String
    let value: String
    
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 24))
                .foregroundColor(iconColor)
            
            VStack(spacing: 4) {
                Text(title)
                    .font(.system(size: 16))
                    .foregroundColor(.white.opacity(0.7))
                
                Text(value)
                    .font(.system(size: 32, weight: .medium))
                    .foregroundColor(.white)
            }
        }
        .frame(minWidth: 140, maxWidth: .infinity)
        .frame(height: 140)
        .background(Color.white.opacity(0.05))
        .clipShape(RoundedRectangle(cornerRadius: 16))
    }
}
