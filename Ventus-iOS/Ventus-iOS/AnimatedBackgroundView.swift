//
//  AnimatedBackgroundView.swift
//  Ventus-iOS
//
//  Created by Karan Haresh Lokchandani on 15/11/24.
//

import SwiftUI

// Create a reusable animated background component
struct AnimatedBackgroundView: View {
    let gradientColors: [Color]
    @State private var rotation: Double = 0
    
    var body: some View {
        RadialGradient(
            gradient: Gradient(colors: gradientColors),
            center: .center,
            startRadius: 100,
            endRadius: 400
        )
        .ignoresSafeArea()
        .overlay(
            GeometryReader { geometry in
                ForEach(0..<20) { index in
                    Circle()
                        .fill(Color.white.opacity(0.1))
                        .frame(width: CGFloat.random(in: 50...150))
                        .position(
                            x: CGFloat.random(in: 0...geometry.size.width),
                            y: CGFloat.random(in: 0...geometry.size.height)
                        )
                        .blur(radius: 20)
                        .rotationEffect(.degrees(rotation))
                        .onAppear {
                            withAnimation(.linear(duration: 8).repeatForever(autoreverses: false)) {
                                rotation = 360
                            }
                        }
                }
            }
        )
    }
}
