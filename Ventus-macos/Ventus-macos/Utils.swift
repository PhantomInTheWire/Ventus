import SwiftUI

// MARK: - UI State Models
class ErrorState: ObservableObject {
    @Published var isShowing = false
    @Published var message = ""
    
    func show(message: String) {
        withAnimation(.spring()) {
            self.message = message
            self.isShowing = true
        }
    }
    
    func clear() {
        withAnimation(.spring()) {
            isShowing = false
            message = ""
        }
    }
}

// MARK: - UI Components
struct AnimatedLogo: View {
    let isShowingInputs: Bool
    
    var body: some View {
        Image(systemName: "network")
            .resizable()
            .scaledToFit()
            .frame(width: 70, height: 70)
            .foregroundStyle(
                LinearGradient(
                    gradient: Gradient(colors: [.blue, .purple]),
                    startPoint: .top,
                    endPoint: .bottom
                )
            )
            .shadow(color: .blue.opacity(0.3), radius: 10, x: 0, y: 5)
            .rotationEffect(Angle(degrees: isShowingInputs ? 360 : 0))
            .animation(.easeInOut(duration: 1).repeatCount(1), value: isShowingInputs)
    }
}

struct AppTitle: View {
    let isShowingInputs: Bool
    
    var body: some View {
        VStack(spacing: 6) {
            Text("Ventus")
                .font(.system(size: 32, weight: .bold))
                .foregroundStyle(
                    LinearGradient(
                        gradient: Gradient(colors: [.blue, .purple]),
                        startPoint: .leading,
                        endPoint: .trailing
                    )
                )
            
            Text("Asynchronous FTP Sync")
                .font(.system(size: 16, weight: .medium))
                .foregroundColor(.white.opacity(0.8))
        }
        .opacity(isShowingInputs ? 1 : 0)
        .offset(y: isShowingInputs ? 0 : 20)
        .animation(.easeOut(duration: 0.6).delay(0.3), value: isShowingInputs)
    }
}

struct CustomInputField: View {
    let icon: String
    let placeholder: String
    @Binding var text: String
    let width: CGFloat
    let showError: Bool
    
    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.white.opacity(0.8))
                .frame(width: 24)
            
            TextField(placeholder, text: $text)
                .foregroundColor(.white)
                .textCase(.none)
                .textFieldStyle(.plain)
                .font(.system(size: 16))
                .frame(maxWidth: width)
            
            if !text.isEmpty {
                ClearButton(text: $text)
            }
        }
        .padding(.horizontal, 16)
        .padding(.vertical, 8)
        .background(InputFieldBackground(showError: showError))
        .padding(.horizontal)
    }
}

struct ClearButton: View {
    @Binding var text: String
    
    var body: some View {
        Button(action: {
            withAnimation(.spring()) {
                text = ""
            }
        }) {
            Image(systemName: "xmark.circle.fill")
                .foregroundColor(.white.opacity(0.6))
                .font(.system(size: 18))
        }
        .buttonStyle(.plain)
        .transition(.scale.combined(with: .opacity))
    }
}

struct InputFieldBackground: View {
    let showError: Bool
    
    var body: some View {
        RoundedRectangle(cornerRadius: 10)
            .fill(Color.white.opacity(0.15))
            .overlay(
                RoundedRectangle(cornerRadius: 10)
                    .stroke(showError ? Color.red.opacity(0.5) : Color.white.opacity(0.2), lineWidth: 1)
            )
            .shadow(color: Color.black.opacity(0.1), radius: 2, x: 0, y: 2)
    }
}

struct ConnectButton: View {
    let isConnecting: Bool
    let connectionProgress: CGFloat
    let isEnabled: Bool
    let action: () -> Void
    private let buttonSize = CGSize(width: 200, height: 40)
    
    var body: some View {
        Button(action: action) {
            ZStack {
                if isConnecting {
                    GeometryReader { geometry in
                        Rectangle()
                            .fill(
                                LinearGradient(
                                    gradient: Gradient(colors: [.blue.opacity(0.3), .purple.opacity(0.3)]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .frame(width: geometry.size.width * connectionProgress)
                            .animation(.easeInOut(duration: 0.3), value: connectionProgress)
                    }
                }
                
                HStack(spacing: 12) {
                    if isConnecting {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                            .scaleEffect(0.8)
                    } else {
                        Image(systemName: "link")
                            .font(.system(size: 18, weight: .bold))
                    }
                    
                    Text(isConnecting ? "Connecting..." : "Connect")
                        .font(.system(size: 16, weight: .bold))
                }
                .frame(width: buttonSize.width, height: buttonSize.height)
            }
        }
        .buttonStyle(StaticButtonStyle())
        .disabled(!isEnabled || isConnecting)
        .background(
            LinearGradient(
                gradient: Gradient(colors: [.blue, .purple]),
                startPoint: .leading,
                endPoint: .trailing
            )
            .opacity(isEnabled ? 1 : 0.5)
            .clipShape(RoundedRectangle(cornerRadius: 8))
        )
        .frame(width: buttonSize.width, height: buttonSize.height)
    }
}


// Custom button style to prevent size changes
struct StaticButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        configuration.label
            .scaleEffect(configuration.isPressed ? 0.98 : 1.0)
            .animation(.easeInOut(duration: 0.2), value: configuration.isPressed)
    }
}

