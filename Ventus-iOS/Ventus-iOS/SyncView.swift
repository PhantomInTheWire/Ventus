import SwiftUI

struct SyncView: View {
    @State private var progress: Float = 0.0
    @State private var rotation: Double = 0.0
    @State private var showingDoneView = false
    @State private var particles: [(offset: CGSize, scale: CGFloat, angle: Double)] = (0..<30).map { _ in
        (CGSize(width: .random(in: -150...150), height: .random(in: -150...150)),
         CGFloat.random(in: 0.2...0.7),
         .random(in: 0...360))
    }
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.1568627451, green: 0.1568627451, blue: 0.4, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Background gradient
                LinearGradient(colors: gradientColors, startPoint: .top, endPoint: .bottom)
                    .ignoresSafeArea()
                
                // Particle system
                ForEach(0..<particles.count, id: \.self) { index in
                    Circle()
                        .fill(.white.opacity(0.5))
                        .frame(width: 4, height: 4)
                        .scaleEffect(particles[index].scale)
                        .offset(particles[index].offset)
                        .rotationEffect(.degrees(particles[index].angle))
                        .blur(radius: 1)
                        .onAppear {
                            let duration = Double.random(in: 2...4)
                            let distance = sqrt(pow(particles[index].offset.width, 2) +
                                                pow(particles[index].offset.height, 2))
                            let endOffset = CGSize(width: particles[index].offset.width * 0.2,
                                                   height: particles[index].offset.height * 0.2)
                            withAnimation(
                                Animation.easeInOut(duration: duration)
                                    .repeatForever(autoreverses: false)
                            ) {
                                particles[index].offset = endOffset
                            }
                            withAnimation(
                                Animation.linear(duration: duration)
                                    .repeatForever(autoreverses: false)
                            ) {
                                particles[index].angle += distance > 0 ? 360 : 0
                            }
                        }
                }
                
                VStack(spacing: 30) {
                    Spacer()
                    
                    Text("Let the Magic Begin...")
                        .font(.system(size: 28, weight: .bold))
                        .foregroundStyle(
                            LinearGradient(
                                colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .shadow(color: .blue.opacity(0.3), radius: 10)
                    
                    // Main progress circle
                    ZStack {
                        Circle()
                            .fill(
                                RadialGradient(
                                    colors: [.blue.opacity(0.2), .clear],
                                    center: .center,
                                    startRadius: 80,
                                    endRadius: 150
                                )
                            )
                            .frame(width: geometry.size.width * 0.7)
                            .blur(radius: 20)
                        
                        ForEach(0..<3) { index in
                            Circle()
                                .stroke(
                                    LinearGradient(
                                        colors: [.blue.opacity(0.2), .purple.opacity(0.1)],
                                        startPoint: .top,
                                        endPoint: .bottom
                                    ),
                                    lineWidth: 1
                                )
                                .frame(width: geometry.size.width * 0.5)
                                .scaleEffect(1 + CGFloat(index) * 0.2)
                                .opacity(1 - Double(progress))
                                .animation(
                                    .easeInOut(duration: 1.5)
                                    .repeatForever(autoreverses: false)
                                    .delay(Double(index) * 0.5),
                                    value: progress
                                )
                        }
                        
                        // Progress track background
                        Circle()
                            .stroke(.white.opacity(0.1), lineWidth: 12)
                            .frame(width: geometry.size.width * 0.5)
                        
                        // Main progress indicator
                        Circle()
                            .trim(from: 0, to: CGFloat(progress))
                            .stroke(
                                LinearGradient(
                                    colors: [.blue, .purple],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                ),
                                style: StrokeStyle(lineWidth: 12, lineCap: .round)
                            )
                            .frame(width: geometry.size.width * 0.5)
                            .rotationEffect(.degrees(-90))
                        
                        // Spinning indicator
                        Circle()
                            .trim(from: 0.4, to: 0.6)
                            .stroke(
                                LinearGradient(
                                    colors: [.blue.opacity(0.5), .purple.opacity(0.5)],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                ),
                                style: StrokeStyle(lineWidth: 2, lineCap: .round)
                            )
                            .frame(width: geometry.size.width * 0.6)
                            .rotationEffect(.degrees(rotation))
                        
                        AnimatedNumberText(
                            number: Int(progress * 100),
                            font: .system(size: 36, weight: .bold),
                            textColor: .white
                        )
                        .overlay(
                            Text("%")
                                .font(.system(size: 20, weight: .bold))
                                .foregroundColor(.white.opacity(0.8))
                                .offset(x: 45, y: -8)
                        )
                    }
                    .frame(height: geometry.size.height * 0.4)
                    
                    // Animated catch phrases below the circle
                    AnimatedPhraseView()
                        .frame(height: 44)
                        .padding(.horizontal)
                    
                    Spacer()
                    
                    NavigationLink(destination: DoneView(), isActive: $showingDoneView) {
                        EmptyView()
                    }
                }
                .padding(.horizontal)
            }
        }
        .onAppear {
            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                rotation = 360
            }
            simulateProgress()
        }
    }
    
    func simulateProgress() {
        Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { timer in
            withAnimation(.easeInOut(duration: 0.2)) {
                if progress < 1.0 {
                    progress += 0.005
                } else {
                    timer.invalidate()
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                        withAnimation {
                            showingDoneView = true
                        }
                    }
                }
            }
        }
    }
}

#Preview {
    SyncView()
}
