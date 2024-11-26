import SwiftUI
import AVFoundation

struct HomeView: View {
    @State private var ipAddress: String = ""
    @State private var folderName: String = ""
    @State private var showingScanner = false
    @State private var showError = false
    @State private var errorMessage = ""
    @State private var isShowingInputs = false
    @State private var isConnecting = false
    @State private var connectionProgress: CGFloat = 0
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    // MARK: - Input Field Style
    private func InputField(icon: String, placeholder: String, text: Binding<String>) -> some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.white.opacity(0.8))
                .frame(width: 24)
            
            TextField(placeholder, text: text)
                .foregroundColor(.white)
                .autocapitalization(.none)
                .disableAutocorrection(true)
                .textContentType(.none)
                .font(.system(size: 16))
                .padding(.vertical, 12)
                .onChange(of: text.wrappedValue) { _ in
                    withAnimation {
                        showError = false
                    }
                }
            
            if !text.wrappedValue.isEmpty {
                Button(action: {
                    withAnimation(.spring()) {
                        text.wrappedValue = ""
                    }
                }) {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.white.opacity(0.6))
                        .font(.system(size: 18))
                }
                .transition(.scale.combined(with: .opacity))
            }
        }
        .padding(.horizontal, 16)
        .background(
            RoundedRectangle(cornerRadius: 15)
                .fill(Color.white.opacity(0.15))
                .overlay(
                    RoundedRectangle(cornerRadius: 15)
                        .stroke(showError ? Color.red.opacity(0.5) : Color.white.opacity(0.2), lineWidth: 1)
                )
                .shadow(color: Color.black.opacity(0.1), radius: 2, x: 0, y: 2)
        )
        .padding(.horizontal)
    }
    
    var body: some View {
        NavigationStack {
            ZStack {
                // Animated Background
                AnimatedBackgroundView(gradientColors: gradientColors)
                    .ignoresSafeArea()
                
                ScrollView {
                    VStack(spacing: 35) {
                        Spacer()
                            .frame(height: 60)
                        
                        // Logo and Title Group
                        VStack(spacing: 18) {
                            // Animated Logo
                            Image(systemName: "network")
                                .resizable()
                                .scaledToFit()
                                .frame(width: 85, height: 85)
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
                            
                            VStack(spacing: 8) {
                                Text("Ventus")
                                    .font(.system(size: 38, weight: .bold))
                                    .foregroundStyle(
                                        LinearGradient(
                                            gradient: Gradient(colors: [.blue, .purple]),
                                            startPoint: .leading,
                                            endPoint: .trailing
                                        )
                                    )
                                
                                Text("Asynchronous FTP Sync")
                                    .font(.system(size: 18, weight: .medium))
                                    .foregroundColor(.white.opacity(0.8))
                            }
                            .opacity(isShowingInputs ? 1 : 0)
                            .offset(y: isShowingInputs ? 0 : 20)
                            .animation(.easeOut(duration: 0.6).delay(0.3), value: isShowingInputs)
                        }
                        .padding(.bottom, 30)
                        
                        // Input Fields Group
                        VStack(spacing: 22) {
                            // IP Address Input with QR Scanner
                            HStack {
                                InputField(icon: "network", placeholder: "", text: $ipAddress)
                                
                                Button(action: { showingScanner = true }) {
                                    Image(systemName: "qrcode.viewfinder")
                                        .foregroundColor(.white)
                                        .font(.system(size: 24))
                                        .frame(width: 48, height: 48)
                                        .background(
                                            LinearGradient(
                                                gradient: Gradient(colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)]),
                                                startPoint: .topLeading,
                                                endPoint: .bottomTrailing
                                            )
                                        )
                                        .clipShape(Circle())
                                        .shadow(color: .black.opacity(0.15), radius: 2, x: 0, y: 2)
                                        .scaleEffect(showingScanner ? 0.95 : 1)
                                        .animation(.spring(), value: showingScanner)
                                }
                                .padding(.trailing, 20)
                            }
                            
                            // Folder Name Input
                            InputField(icon: "folder", placeholder: "", text: $folderName)
                        }
                        .opacity(isShowingInputs ? 1 : 0)
                        .offset(y: isShowingInputs ? 0 : 20)
                        
                        // Navigation to SyncView with Connect Button
                        NavigationLink(
                            destination: SyncView(ipAddress: ipAddress, localDir: folderName),
                            isActive: Binding(
                                get: { isConnecting && connectionProgress >= 1.0 },
                                set: { _ in }
                            )
                        ) {
                            Button(action: validateAndConnect) {
                                ZStack {
                                    // Progress Bar
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
                                        } else {
                                            Image(systemName: "link")
                                                .font(.system(size: 20, weight: .bold))
                                        }
                                        
                                        Text(isConnecting ? "Connecting..." : "Connect")
                                            .font(.system(size: 20, weight: .bold))
                                    }
                                    .padding()
                                    .frame(maxWidth: .infinity)
                                    .background(
                                        LinearGradient(
                                            gradient: Gradient(colors: [.blue, .purple]),
                                            startPoint: .leading,
                                            endPoint: .trailing
                                        )
                                        .opacity(isValidInput ? 1 : 0.5)
                                    )
                                    .foregroundColor(.white)
                                    .cornerRadius(20)
                                }
                            }
                        }
                        .disabled(!isValidInput || isConnecting)
                        .padding(.horizontal, 20)
                        .padding(.top, 25)
                        .opacity(isShowingInputs ? 1 : 0)
                        .offset(y: isShowingInputs ? 0 : 20)
                        
                        // Error Message
                        if showError {
                            HStack(spacing: 8) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundColor(.red.opacity(0.9))
                                Text(errorMessage)
                                    .foregroundColor(.red.opacity(0.9))
                                    .font(.system(size: 14, weight: .medium))
                            }
                            .padding(.horizontal, 20)
                            .padding(.top, 12)
                            .transition(.opacity.combined(with: .scale))
                        }
                        
                        Spacer()
                    }
                    .padding(.top, 40)
                }
            }
            .navigationBarHidden(true)
            .sheet(isPresented: $showingScanner) {
                QRCodeScannerView { scannedCode in
                    showingScanner = false
                    withAnimation(.spring()) {
                        ipAddress = scannedCode
                    }
                }
            }
            .onAppear {
                withAnimation(.easeOut(duration: 0.8).delay(0.2)) {
                    isShowingInputs = true
                }
            }
        }
    }
    
    // MARK: - Computed Properties
    private var isValidInput: Bool {
        isValidIP(ipAddress) && !folderName.isEmpty
    }
    
    // MARK: - Methods
    private func validateAndConnect() {
        withAnimation(.spring()) {
            if !isValidIP(ipAddress) {
                showError = true
                errorMessage = "Please enter a valid IP address"
                return
            }
            
            if folderName.isEmpty {
                showError = true
                errorMessage = "Please enter a folder name"
                return
            }
            
            showError = false
            isConnecting = true
            
            // Simulate connection progress
            withAnimation {
                connectionProgress = 0.3
            }
            
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                withAnimation {
                    connectionProgress = 0.7
                }
            }
            
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
                withAnimation {
                    connectionProgress = 1.0
                }
            }
        }
    }
    
    private func isValidIP(_ ip: String) -> Bool {
        let parts = ip.split(separator: ".")
        guard parts.count == 4 else { return false }
        
        return parts.allSatisfy { part in
            guard let number = Int(part) else { return false }
            return number >= 0 && number <= 255
        }
    }
}

#Preview {
    HomeView()
}
