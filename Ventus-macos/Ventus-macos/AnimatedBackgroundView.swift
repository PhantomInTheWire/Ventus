//
//  AnimatedBackgroundView.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 18/11/24.
//


import SwiftUI

struct AnimatedBackgroundView: View {
    let gradientColors: [Color]
    
    // Increased number of particles for larger desktop screens
    @State private var particles: [(offset: CGSize, scale: CGFloat, opacity: CGFloat)] = (0..<8).map { _ in
        (
            CGSize(width: .random(in: -150...150), height: .random(in: -150...150)),
            CGFloat.random(in: 0.6...1.2),
            CGFloat.random(in: 0.1...0.2)
        )
    }
    
    // Add hover state for interactive effects
    @State private var isHovered = false
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Background gradient
                LinearGradient(colors: gradientColors, startPoint: .top, endPoint: .bottom)
                    .ignoresSafeArea()
                
                // Subtle Particle system
                ForEach(0..<particles.count, id: \.self) { index in
                    Circle()
                        .fill(.white.opacity(particles[index].opacity))
                        .frame(width: 25, height: 25)
                        .scaleEffect(particles[index].scale * (isHovered ? 1.1 : 1.0))
                        .offset(particles[index].offset)
                        .blur(radius: 10)
                        .onAppear {
                            // Create smooth, continuous animation
                            let baseAnimation = Animation
                                .easeInOut(duration: Double.random(in: 4...7))
                                .repeatForever(autoreverses: true)
                            
                            // Animate position
                            withAnimation(baseAnimation) {
                                particles[index].offset = CGSize(
                                    width: .random(in: -geometry.size.width/4...geometry.size.width/4),
                                    height: .random(in: -geometry.size.height/4...geometry.size.height/4)
                                )
                            }
                            
                            // Animate scale
                            withAnimation(
                                Animation
                                    .easeInOut(duration: Double.random(in: 3...5))
                                    .repeatForever(autoreverses: true)
                            ) {
                                particles[index].scale = CGFloat.random(in: 0.8...1.4)
                            }
                            
                            // Animate opacity
                            withAnimation(
                                Animation
                                    .easeInOut(duration: Double.random(in: 2...4))
                                    .repeatForever(autoreverses: true)
                            ) {
                                particles[index].opacity = CGFloat.random(in: 0.08...0.15)
                            }
                        }
                }
            }
            .onHover { hovering in
                withAnimation(.easeInOut(duration: 0.3)) {
                    isHovered = hovering
                }
            }
        }
    }
}

// Preview
#Preview {
    AnimatedBackgroundView(gradientColors: [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ])
}