//
//  ParticleView.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct ParticleView: View {
    let particle: ParticleModel
    @State private var animatedOffset: CGSize
    
    init(particle: ParticleModel) {
        self.particle = particle
        self._animatedOffset = State(
            initialValue: CGSize(
                width: particle.offset.width * 0.3,
                height: particle.offset.height * 0.3
            )
        )
    }
    
    var body: some View {
        Group {
            // Outer circle for larger effect
            Circle()
                .fill(.white.opacity(particle.opacity))
                .frame(width: 10, height: 10) // Increased size
                .blur(radius: 2) // Slightly more blur for a glowing effect
            
            // Inner circle for depth
            Circle()
                .fill(.white.opacity(particle.opacity * 0.3))
                .frame(width: 6, height: 6) // Increased size
                .blur(radius: 4)
                .offset(x: -animatedOffset.width * 0.2, y: -animatedOffset.height * 0.2)
        }
        .scaleEffect(particle.scale * 1.5) // Scaling up the particle size further
        .offset(animatedOffset)
        .rotationEffect(.degrees(particle.angle))
        .onAppear {
            withAnimation(
                .easeInOut(duration: Double.random(in: 3...5))
                .repeatForever(autoreverses: true)
            ) {
                animatedOffset = particle.offset
            }
        }
    }
}
