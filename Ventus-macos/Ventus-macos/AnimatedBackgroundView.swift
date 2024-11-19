import SwiftUI

struct AnimatedBackgroundView: View {
    let gradientColors: [Color]
    
    // Increased number of particles and wider initial distribution
    @State private var particles: [(offset: CGSize, scale: CGFloat, opacity: CGFloat)] = (0..<12).map { _ in
        (
            CGSize(
                width: .random(in: -250...250),
                height: .random(in: -250...250)
            ),
            CGFloat.random(in: 0.6...1.4),
            CGFloat.random(in: 0.1...0.2)
        )
    }
    
    @State private var isHovered = false
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Background gradient
                LinearGradient(colors: gradientColors, startPoint: .top, endPoint: .bottom)
                    .ignoresSafeArea()
                
                // Enhanced Particle system
                ForEach(0..<particles.count, id: \.self) { index in
                    Circle()
                        .fill(.white.opacity(particles[index].opacity))
                        .frame(width: 25, height: 25)
                        .scaleEffect(particles[index].scale * (isHovered ? 1.1 : 1.0))
                        .offset(particles[index].offset)
                        .blur(radius: 10)
                        .onAppear {
                            // Randomize animation timing for more natural movement
                            let animationDelay = Double.random(in: 0...2)
                            
                            DispatchQueue.main.asyncAfter(deadline: .now() + animationDelay) {
                                // Create wider movement range based on screen size
                                let baseAnimation = Animation
                                    .easeInOut(duration: Double.random(in: 6...10))
                                    .repeatForever(autoreverses: true)
                                
                                // Animate position with wider range
                                withAnimation(baseAnimation) {
                                    particles[index].offset = CGSize(
                                        width: .random(in: -geometry.size.width/2...geometry.size.width/2),
                                        height: .random(in: -geometry.size.height/2...geometry.size.height/2)
                                    )
                                }
                                
                                // Slower scale animation for smoother effect
                                withAnimation(
                                    Animation
                                        .easeInOut(duration: Double.random(in: 4...7))
                                        .repeatForever(autoreverses: true)
                                ) {
                                    particles[index].scale = CGFloat.random(in: 0.7...1.6)
                                }
                                
                                // Subtle opacity changes
                                withAnimation(
                                    Animation
                                        .easeInOut(duration: Double.random(in: 3...5))
                                        .repeatForever(autoreverses: true)
                                ) {
                                    particles[index].opacity = CGFloat.random(in: 0.06...0.18)
                                }
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
    AnimatedBackgroundView(gradientColors: [Color.blue, Color.indigo])
}
