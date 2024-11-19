import SwiftUI

struct AnimatedNumberText: View {
    let number: Int
    let font: Font
    let textColor: Color
    
    @State private var animationValue: CGFloat = 0
    @State private var glitchOffset: CGFloat = 0
    @State private var isGlitching = false
    @State private var hologramOpacity: CGFloat = 0.5
    @State private var scanlineOffset: CGFloat = 0
    
    var body: some View {
        ZStack {
            // Cyber background glow
            Text("\(number)")
                .font(font)
                .foregroundColor(textColor.opacity(0.3))
                .blur(radius: 15)
                .scaleEffect(1.2)
            
            // Main hologram number
            Text("\(number)")
                .font(font)
                .foregroundStyle(
                    LinearGradient(
                        colors: [
                            textColor.opacity(0.8),
                            textColor.opacity(0.9),
                            textColor.opacity(0.4)
                        ],
                        startPoint: .top,
                        endPoint: .bottom
                    )
                )
                .overlay(
                    // Hologram scan line
                    Rectangle()
                        .fill(
                            LinearGradient(
                                colors: [.clear, textColor.opacity(0.3), .clear],
                                startPoint: .top,
                                endPoint: .bottom
                            )
                        )
                        .frame(height: 50)
                        .offset(y: scanlineOffset)
                        .mask(
                            Text("\(number)")
                                .font(font)
                        )
                )
                .overlay(
                    // Digital noise effect
                    GeometryReader { geometry in
                        ZStack {
                            ForEach(0..<20) { _ in
                                Rectangle()
                                    .fill(textColor)
                                    .opacity(isGlitching ? 0.1 : 0)
                                    .frame(
                                        width: CGFloat.random(in: 1...5),
                                        height: CGFloat.random(in: 1...3)
                                    )
                                    .offset(
                                        x: CGFloat.random(in: -geometry.size.width/2...geometry.size.width/2),
                                        y: CGFloat.random(in: -geometry.size.height/2...geometry.size.height/2)
                                    )
                            }
                        }
                    }
                )
                .modifier(GlitchModifier(isGlitching: isGlitching))
                .scaleEffect(1.0 + sin(animationValue) * 0.1)
        }
        .onChange(of: number) { oldValue, newValue in
            // Trigger animations
            withAnimation(.spring(response: 0.3, dampingFraction: 0.6)) {
                animationValue = .pi
                isGlitching = true
            }
            
            // Reset animations
            withAnimation(.spring(response: 0.3, dampingFraction: 0.6).delay(0.1)) {
                animationValue = 0
                isGlitching = false
            }
            
            // Continuous hologram scan animation
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                scanlineOffset = 50
            }
        }
        .onAppear {
            // Start continuous animations
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: true)) {
                hologramOpacity = 0.8
            }
            
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                scanlineOffset = 50
            }
        }
    }
}

// Glitch effect modifier
struct GlitchModifier: ViewModifier {
    let isGlitching: Bool
    
    func body(content: Content) -> some View {
        content
            .overlay(
                GeometryReader { geometry in
                    if isGlitching {
                        content
                            .foregroundColor(.blue.opacity(0.5))
                            .offset(x: 2, y: -2)
                            .mask(
                                Rectangle()
                                    .offset(y: geometry.size.height * 0.35)
                                    .frame(height: geometry.size.height * 0.3)
                            )
                        
                        content
                            .foregroundColor(.red.opacity(0.5))
                            .offset(x: -2, y: 2)
                            .mask(
                                Rectangle()
                                    .offset(y: -geometry.size.height * 0.35)
                                    .frame(height: geometry.size.height * 0.3)
                            )
                    }
                }
            )
    }
}

// Preview
struct AnimatedNumberText_Previews: PreviewProvider {
    static var previews: some View {
        ZStack {
            Color.black
            AnimatedNumberText(
                number: 85,
                font: .system(size: 32, weight: .bold),
                textColor: .white
            )
        }
    }
}
