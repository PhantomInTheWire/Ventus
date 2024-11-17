import SwiftUI

struct DoneView: View {
    @State private var isShowingContent = false
    @State private var navigateToHome = false // State for navigation

    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]

    var body: some View {
        NavigationView {
            ZStack {
                // Background
                AnimatedBackgroundView(gradientColors: gradientColors)
                    .ignoresSafeArea()

                // Main Content
                VStack(spacing: 20) {
                    Spacer().frame(height: 40)

                    SuccessIcon(size: 100, isShowing: $isShowingContent)

                    SuccessTitle()

                    StatsSection(isPortrait: true, geometry: UIScreen.main.bounds, isShowing: $isShowingContent)

                    // Directly using the BackToHomeButton without the NavigationLink here
                    BackToHomeButton(navigateToHome: $navigateToHome) // Pass binding
                        .padding(.horizontal, 40)
                        .padding(.top, 20)
                        .background(
                            NavigationLink(destination: HomeView(), isActive: $navigateToHome) {
                                EmptyView()
                            }
                            .hidden()
                        )
                }
                .padding()
                .offset(y: isShowingContent ? 0 : UIScreen.main.bounds.height * 0.3) // Start from below
                .opacity(isShowingContent ? 1 : 0) // Fade in
                .animation(.easeOut(duration: 0.8), value: isShowingContent) // Smooth animation
            }
            .navigationBarHidden(true)
            .onAppear {
                withAnimation {
                    isShowingContent = true
                }
            }
        }
    }
}

struct SuccessIcon: View {
    let size: CGFloat
    @Binding var isShowing: Bool
    @State private var scale: CGFloat = 0.5
    @State private var opacity: Double = 0
    @State private var rotation: Double = 0
    @State private var gradientStart: UnitPoint = .leading
    @State private var gradientEnd: UnitPoint = .trailing

    var body: some View {
        Image(systemName: "checkmark.circle.fill")
            .resizable()
            .frame(width: size, height: size)
            .foregroundStyle(
                LinearGradient(
                    gradient: Gradient(colors: [Color.purple, Color.blue]),
                    startPoint: gradientStart,
                    endPoint: gradientEnd
                )
            )
            .shadow(color: .purple.opacity(0.4), radius: 10)
            .scaleEffect(scale)
            .opacity(opacity)
            .rotationEffect(.degrees(rotation))
            .onAppear {
                // Animation: Scale and Opacity
                withAnimation(.spring(response: 0.6, dampingFraction: 0.6, blendDuration: 0.5)) {
                    scale = 1.0
                }
                withAnimation(.easeInOut(duration: 0.6)) {
                    opacity = 1.0
                }

                // Animation: Gradient
                withAnimation(.linear(duration: 2).repeatForever(autoreverses: true)) {
                    gradientStart = .bottom
                    gradientEnd = .top
                }
            }
    }
}

struct SuccessTitle: View {
    var body: some View {
        Text("Sync Complete!")
            .font(.largeTitle)
            .fontWeight(.bold)
            .foregroundColor(.white)
    }
}

struct StatsSection: View {
    let isPortrait: Bool
    let geometry: CGRect
    @Binding var isShowing: Bool

    var body: some View {
        VStack(spacing: 20) {
            HStack {
                StatCard(icon: "clock.fill", title: "Time Taken", value: "2m 34s")
                StatCard(icon: "calendar", title: "Last Sync", value: "Just now")
            }
            VStack(spacing: 10) {
                ForEach(["Files synced: 128", "Total size: 1.2 GB", "Status: Complete"], id: \.self) { detail in
                    HStack {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                        Text(detail)
                            .foregroundColor(.white)
                    }
                }
            }
        }
    }
}

struct BackToHomeButton: View {
    @Binding var navigateToHome: Bool

    var body: some View {
        Button(action: {
            navigateToHome = true // Trigger navigation
        }) {
            Text("Back to Home")
                .font(.headline)
                .padding()
                .frame(maxWidth: .infinity)
                .background(LinearGradient(colors: [Color.blue, Color.purple], startPoint: .leading, endPoint: .trailing))
                .foregroundColor(.white)
                .cornerRadius(10)
        }
    }
}

#Preview {
    DoneView()
}
