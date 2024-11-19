import SwiftUI

struct SyncStats {
    var filesTransferred: Int
    var totalSize: Double // in MB
    var averageSpeed: Double // in MB/s
    var timeElapsed: TimeInterval
}

struct DoneView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var stats = SyncStats(
        filesTransferred: 128,
        totalSize: 256.5,
        averageSpeed: 12.3,
        timeElapsed: 45
    )
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Animated Background
                AnimatedBackgroundView(gradientColors: gradientColors)
                    .ignoresSafeArea()
                
                ScrollView {
                    VStack(spacing: geometry.size.height * 0.05) {
                        // Success Icon
                        Circle()
                            .fill(.green)
                            .frame(width: min(geometry.size.width * 0.15, 80),
                                  height: min(geometry.size.width * 0.15, 80))
                            .overlay(
                                Image(systemName: "checkmark")
                                    .font(.system(size: min(geometry.size.width * 0.08, 40)))
                                    .foregroundColor(.white)
                            )
                            .padding(.top, geometry.size.height * 0.05)
                        
                        // Title and Subtitle
                        VStack(spacing: 8) {
                            Text("Sync Complete!")
                                .font(.system(size: min(geometry.size.width * 0.1, 40), weight: .bold))
                                .foregroundStyle(
                                    LinearGradient(
                                        gradient: Gradient(colors: [.blue, .purple]),
                                        startPoint: .leading,
                                        endPoint: .trailing
                                    )
                                )
                            
                            Text("All files have been synchronized successfully")
                                .font(.system(size: min(geometry.size.width * 0.04, 16)))
                                .foregroundColor(.white.opacity(0.8))
                        }
                        
                        // Stats Grid
                        let gridColumns = [
                            GridItem(.adaptive(minimum: min(geometry.size.width * 0.4, 200), maximum: .infinity), spacing: geometry.size.width * 0.05)
                        ]
                        
                        LazyVGrid(columns: gridColumns, spacing: geometry.size.height * 0.03) {
                            // Files Transferred
                            StatBox(
                                icon: "doc.fill",
                                iconColor: .blue,
                                title: "Files Transferred",
                                value: "\(stats.filesTransferred)"
                            )
                            
                            // Total Size
                            StatBox(
                                icon: "externaldrive.fill",
                                iconColor: .purple,
                                title: "Total Size",
                                value: String(format: "%.1f MB", stats.totalSize)
                            )
                            
                            // Average Speed
                            StatBox(
                                icon: "speedometer",
                                iconColor: .orange,
                                title: "Average Speed",
                                value: String(format: "%.1f MB/s", stats.averageSpeed)
                            )
                            
                            // Time Elapsed
                            StatBox(
                                icon: "clock.fill",
                                iconColor: .green,
                                title: "Time Elapsed",
                                value: "\(Int(stats.timeElapsed))s"
                            )
                        }
                        .padding(.horizontal, geometry.size.width * 0.05)
                        
                        // Updated Back to Home Button
                        NavigationLink(destination: HomeView()) {
                            HStack(spacing: 12) {
                                Image(systemName: "house.fill")
                                    .font(.system(size: 20))
                                Text("Back to Home")
                                    .font(.system(size: 18, weight: .semibold))
                            }
                            .foregroundColor(.white)
                            .frame(width: min(geometry.size.width * 0.6, 250), height: 50)
                            .background(
                                LinearGradient(
                                    gradient: Gradient(colors: [
                                        Color.blue,
                                        Color.purple
                                    ]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .clipShape(RoundedRectangle(cornerRadius: 25))
                            .shadow(color: .purple.opacity(0.3), radius: 10, x: 0, y: 5)
                        }
                        .buttonStyle(PlainButtonStyle())
                        .padding(.top, geometry.size.height * 0.02)
                    }
                    .frame(minHeight: geometry.size.height)
                }
            }
        }
    }
}

#Preview {
    DoneView()
        .frame(minWidth: 600, minHeight: 600)
}
