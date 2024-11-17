//import SwiftUI
//
//struct SuccessIcon: View {
//    let size: CGFloat
//    @Binding var isShowing: Bool
//    @State private var rotation: Double = 0
//    
//    var body: some View {
//        Image(systemName: "checkmark.circle.fill")
//            .resizable()
//            .scaledToFit()
//            .frame(width: size)
//            .foregroundStyle(
//                LinearGradient(
//                    gradient: Gradient(colors: [Color.blue, Color.purple]),
//                    startPoint: .top,
//                    endPoint: .bottom
//                )
//            )
//            .shadow(color: .blue.opacity(0.4), radius: 15, x: 0, y: 8)
//            .rotationEffect(Angle(degrees: rotation))
//            .overlay(ParticleEffect())
//            .onAppear {
//                withAnimation(.spring(response: 0.6, dampingFraction: 0.5, blendDuration: 0.5)) {
//                    rotation = 360
//                }
//            }
//    }
//}
//
//struct SuccessTitle: View {
//    @State private var offset: CGFloat = 1000
//    
//    var body: some View {
//        Text("Sync Complete!")
//            .font(.system(size: 34, weight: .bold, design: .rounded))
//            .foregroundStyle(
//                LinearGradient(
//                    gradient: Gradient(colors: [Color.blue, Color.purple]),
//                    startPoint: .leading,
//                    endPoint: .trailing
//                )
//            )
//            .offset(y: offset)
//            .onAppear {
//                withAnimation(.spring(response: 0.8, dampingFraction: 0.6, blendDuration: 0.6)) {
//                    offset = 0
//                }
//            }
//    }
//}
//
//struct StatsSection: View {
//    let isPortrait: Bool
//    let geometry: GeometryProxy
//    @Binding var isShowing: Bool
//    
//    var body: some View {
//        VStack(spacing: 20) {
//            HStack(spacing: 20) {
//                StatCard(icon: "clock.fill", title: "Time Taken", value: "2m 34s")
//                StatCard(icon: "calendar", title: "Last Sync", value: "Just now")
//            }
//            .padding(.horizontal, geometry.size.width * 0.05)
//            
//            VStack(spacing: 15) {
//                ForEach(["Files synced: 128", "Total size: 1.2 GB", "Status: Complete"], id: \.self) { detail in
//                    HStack {
//                        Image(systemName: "checkmark.circle.fill")
//                            .foregroundColor(.green)
//                        Text(detail)
//                            .foregroundColor(.white)
//                    }
//                    .transition(.asymmetric(insertion: .scale.combined(with: .opacity), removal: .opacity))
//                }
//            }
//            .padding(.horizontal, 20)
//            .padding(.vertical, 10)
//            .background(
//                RoundedRectangle(cornerRadius: 15)
//                    .fill(Color.black.opacity(0.15))
//            )
//        }
//        .opacity(isShowing ? 1 : 0)
//        .scaleEffect(isShowing ? 1 : 0.95)
//        .animation(.spring(response: 0.6, dampingFraction: 0.7, blendDuration: 0.5).delay(0.4), value: isShowing)
//    }
//}
//
//struct BackToHomeButton: View {
//    @Binding var navigateToHome: Bool
//    @State private var isPressed = false
//    
//    var body: some View {
//        NavigationLink(destination: HomeView(), isActive: $navigateToHome) {
//            Button(action: {
//                withAnimation(.spring()) {
//                    isPressed = true
//                }
//                DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
//                    navigateToHome = true
//                }
//            }) {
//                HStack {
//                    Image(systemName: "house.fill")
//                    Text("Back to Home")
//                }
//                .font(.headline)
//                .padding()
//                .frame(maxWidth: .infinity)
//                .background(
//                    LinearGradient(
//                        gradient: Gradient(colors: [Color.blue, Color.purple]),
//                        startPoint: .leading,
//                        endPoint: .trailing
//                    )
//                )
//                .foregroundColor(.white)
//                .cornerRadius(15)
//                .shadow(color: .purple.opacity(0.3), radius: 10, x: 0, y: 5)
//                .scaleEffect(isPressed ? 0.95 : 1)
//            }
//        }
//    }
//}
//
//struct ParticleEffect: View {
//    @State private var time: Double = 0
//    
//    var body: some View {
//        TimelineView(.animation) { timeline in
//            Canvas { context, size in
//                let timeNow = timeline.date.timeIntervalSinceReferenceDate
//                time = timeNow.remainder(dividingBy: 2)
//                
//                context.addFilter(.blur(radius: 5))
//                
//                for _ in 0..<15 {
//                    let position = CGPoint(
//                        x: .random(in: 0...size.width),
//                        y: .random(in: 0...size.height)
//                    )
//                    let size = CGSize(width: .random(in: 2...5), height: .random(in: 2...5))
//                    let opacity = Double.random(in: 0.1...0.5)
//                    let hue = Double.random(in: 0...1)
//                    
//                    context.opacity = opacity
//                    context.fill(
//                        Path(ellipseIn: CGRect(origin: position, size: size)),
//                        with: .color(Color(hue: hue, saturation: 1, brightness: 1))
//                    )
//                }
//            }
//        }
//        .allowsHitTesting(false)
//    }
//}
