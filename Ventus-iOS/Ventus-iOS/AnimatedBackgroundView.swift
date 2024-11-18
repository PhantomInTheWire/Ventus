import SwiftUI

// Create a reusable animated background component
struct AnimatedBackgroundView: View {
    let gradientColors: [Color]
    @State private var particles: [(offset: CGSize, scale: CGFloat)] = (0..<6).map { _ in
        (CGSize(width: .random(in: -200...200), height: .random(in: -200...200)), // Increased range
         CGFloat.random(in: 0.6...1.0))
    }
    
    var body: some View {
        ZStack {
            // Background gradient
            LinearGradient(colors: gradientColors, startPoint: .top, endPoint: .bottom)
                .ignoresSafeArea()
            
            // Subtle Particle system
            ForEach(0..<particles.count, id: \.self) { index in
                Circle()
                    .fill(.white.opacity(0.15)) // Reduced opacity for a softer effect
                    .frame(width: 60, height: 60) // Tripled particle size
                    .scaleEffect(particles[index].scale)
                    .offset(particles[index].offset)
                    .blur(radius: 8) // Increased blur for a more subtle look
                    .onAppear {
                        withAnimation(
                            .easeInOut(duration: 5)
                            .repeatForever(autoreverses: true)
                        ) {
                            particles[index].offset = CGSize(
                                width: .random(in: -300...300), // Further increased range
                                height: .random(in: -300...300)
                            )
                        }
                    }
            }
        }
    }
}
