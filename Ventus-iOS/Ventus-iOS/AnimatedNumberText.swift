//
//  AnimatedNumberText.swift
//  Ventus-iOS
//
//  Created by Karan Haresh Lokchandani on 18/11/24.
//
import SwiftUI

struct AnimatedNumberText: View {
    let number: Int
    let font: Font
    let textColor: Color
    @State private var animationValue: CGFloat = 0
    
    var body: some View {
        HStack(spacing: 0) {
            Text("\(number)")
                .font(font)
                .foregroundColor(textColor)
                .transition(.scale.combined(with: .opacity))
                .scaleEffect(1.0 + sin(animationValue) * 0.1)
                .shadow(color: .blue.opacity(0.5), radius: 10, y: 5)
        }
        .onChange(of: number) { oldValue, newValue in
            withAnimation(.spring(response: 0.3, dampingFraction: 0.6)) {
                animationValue = .pi
            }
            withAnimation(.spring(response: 0.3, dampingFraction: 0.6).delay(0.1)) {
                animationValue = 0
            }
        }
    }
}
