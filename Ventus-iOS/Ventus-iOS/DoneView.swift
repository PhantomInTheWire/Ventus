import SwiftUI

struct DoneView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var navigateToHome = false
    @State private var showConfetti = false
    @State private var scale: CGFloat = 0.8
    @State private var checkmarkScale: CGFloat = 0
    @State private var checkmarkOpacity: Double = 0
    @State private var checkmarkRotation: Double = -360
    @State private var backgroundRotation: Double = 0
    @State private var circleScale: CGFloat = 0.8
    @State private var circleOpacity: Double = 0
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.1, green: 0.1, blue: 0.3, alpha: 1))
    ]
    
    var body: some View {
        NavigationStack {
            ZStack {
                // Background with radial gradient
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
                                .rotationEffect(.degrees(Double(index) * 18 + backgroundRotation))
                        }
                    }
                )
                
                VStack(spacing: 40) {
                    // Success icon with animation
                    ZStack {
                        // Outer glow
                        Circle()
                            .fill(Color.white.opacity(0.1))
                            .frame(width: 200, height: 200)
                            .blur(radius: 20)
                            .scaleEffect(circleScale)
                            .opacity(circleOpacity)
                        
                        // Animated circle border
                        Circle()
                            .stroke(
                                LinearGradient(
                                    gradient: Gradient(colors: [.blue, .purple, .teal]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                ),
                                lineWidth: 4
                            )
                            .frame(width: 160, height: 160)
                            .scaleEffect(circleScale)
                            .opacity(circleOpacity)
                        
                        // Checkmark with gradient and animations
                        Image(systemName: "checkmark.circle.fill")
                            .resizable()
                            .scaledToFit()
                            .frame(width: 120, height: 120)
                            .foregroundStyle(
                                LinearGradient(
                                    gradient: Gradient(colors: [.blue, .purple, .teal]),
                                    startPoint: .top,
                                    endPoint: .bottom
                                )
                            )
                            .scaleEffect(checkmarkScale)
                            .opacity(checkmarkOpacity)
                            .rotationEffect(.degrees(checkmarkRotation))
                    }
                    .padding(40)
                    
                    // Success message
                    VStack(spacing: 20) {
                        Text("Sync Complete!")
                            .font(.system(size: 36, weight: .bold))
                            .foregroundStyle(
                                LinearGradient(
                                    gradient: Gradient(colors: [Color.blue, Color.purple, Color.teal]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .shadow(color: .black.opacity(0.2), radius: 10, x: 0, y: 5)
                            .opacity(checkmarkOpacity)
                            .scaleEffect(checkmarkScale)
                        
                        Text("All files have been successfully synchronized")
                            .font(.system(size: 18, weight: .medium))
                            .foregroundColor(.white.opacity(0.8))
                            .multilineTextAlignment(.center)
                            .padding(.horizontal)
                            .opacity(checkmarkOpacity)
                            .scaleEffect(checkmarkScale)
                    }
                    
                    // Button with navigation to HomeView
                    NavigationLink(destination: HomeView(), isActive: $navigateToHome) {
                        Button(action: {
                            // Animate out before navigating
                            withAnimation(.easeInOut(duration: 0.3)) {
                                checkmarkScale = 0
                                checkmarkOpacity = 0
                                circleScale = 0
                                circleOpacity = 0
                            }
                            DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                                navigateToHome = true
                            }
                        }) {
                            HStack {
                                Image(systemName: "house.fill")
                                    .font(.system(size: 20, weight: .bold))
                                Text("Back to Home")
                                    .font(.system(size: 20, weight: .bold))
                            }
                            .padding()
                            .frame(maxWidth: .infinity)
                            .background(
                                LinearGradient(
                                    gradient: Gradient(colors: [Color.blue, Color.purple]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .foregroundColor(.white)
                            .cornerRadius(20)
                            .shadow(color: Color.blue.opacity(0.5), radius: 10, x: 0, y: 5)
                        }
                    }
                    .padding(.horizontal)
                    .padding(.top, 20)
                    .opacity(checkmarkOpacity)
                    .scaleEffect(checkmarkScale)
                }
                .padding()
            }
            .onAppear {
                // Start the background rotation animation
                withAnimation(.linear(duration: 8).repeatForever(autoreverses: false)) {
                    backgroundRotation = 360
                }
                
                // Animate in the circle elements
                withAnimation(.spring(response: 0.6, dampingFraction: 0.7)) {
                    circleScale = 1.0
                    circleOpacity = 1.0
                }
                
                // Animate in the checkmark and text elements with a slight delay
                DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                    withAnimation(.spring(response: 0.8, dampingFraction: 0.7)) {
                        checkmarkScale = 1.0
                        checkmarkOpacity = 1.0
                        checkmarkRotation = 0
                    }
                }
            }
        }
    }
}
