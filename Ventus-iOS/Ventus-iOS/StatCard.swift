//
//  StatCard.swift
//  Ventus-iOS
//
//  Created by Karan Haresh Lokchandani on 18/11/24.
//
import SwiftUI

struct StatCard: View {
    let icon: String
    let title: String
    let value: String
    
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 24))
                .foregroundColor(.blue)
            
            VStack(spacing: 4) {
                Text(title)
                    .font(.system(size: 14))
                    .foregroundColor(.gray)
                Text(value)
                    .font(.system(size: 16, weight: .bold))
                    .foregroundColor(.white)
            }
        }
        .frame(maxWidth: .infinity)
        .frame(height: 100) // Fixed height
        .padding()
        .background(Color.black.opacity(0.3))
        .cornerRadius(15)
    }
}
