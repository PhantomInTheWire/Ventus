import SwiftUI
import AVFoundation

struct HomeView: View {
    @State private var ipAddress: String = ""
    @State private var folderName: String = ""
    @State private var showingSyncView = false
    @State private var showingScanner = false // Controls QR scanner display
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.1, green: 0.1, blue: 0.3, alpha: 1))
    ]
    
    var body: some View {
        NavigationView {
            ZStack {
                AnimatedBackgroundView(gradientColors: gradientColors)
                
                VStack(spacing: 30) {
                    Spacer()
                    
                    Image(systemName: "network")
                        .resizable()
                        .scaledToFit()
                        .frame(width: 100, height: 100)
                        .foregroundStyle(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.blue, Color.purple]),
                                startPoint: .top,
                                endPoint: .bottom
                            )
                        )
                    
                    Text("Ventus")
                        .font(.system(size: 36, weight: .bold))
                        .foregroundStyle(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.blue, Color.purple, Color.teal]),
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                    
                    Text("Asynchronous FTP Sync")
                        .font(.system(size: 18, weight: .medium))
                        .foregroundColor(.white.opacity(0.8))
                    
                    // IP Address Input
                    HStack {
                        Image(systemName: "network")
                            .foregroundColor(.white.opacity(0.8))
                        
                        TextField("Enter IP Address", text: $ipAddress)
                            .foregroundColor(.white.opacity(0.9))
                            .padding()
                            .keyboardType(.numbersAndPunctuation)
                        
                        Button(action: { showingScanner = true }) {
                            Image(systemName: "qrcode.viewfinder")
                                .foregroundColor(.blue)
                                .font(.system(size: 24))
                        }
                    }
                    .padding()
                    .background(
                        RoundedRectangle(cornerRadius: 15)
                            .fill(Color.white.opacity(0.2))
                    )
                    .padding(.horizontal)
                    
                    // Folder Name Input
                    HStack {
                        Image(systemName: "folder")
                            .foregroundColor(.white.opacity(0.8))
                        
                        TextField("Enter Folder Name", text: $folderName)
                            .foregroundColor(.white.opacity(0.9))
                            .padding()
                    }
                    .padding()
                    .background(
                        RoundedRectangle(cornerRadius: 15)
                            .fill(Color.white.opacity(0.2))
                    )
                    .padding(.horizontal)
                    
                    // Connect Button
                    NavigationLink(destination: SyncView(), isActive: $showingSyncView) {
                        Button(action: {
                            if isValidIP(ipAddress) && !folderName.isEmpty {
                                showingSyncView = true
                            }
                        }) {
                            HStack {
                                Image(systemName: "link")
                                    .font(.system(size: 20, weight: .bold))
                                Text("Connect")
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
                                .opacity(isValidIP(ipAddress) && !folderName.isEmpty ? 1 : 0.5)
                            )
                            .foregroundColor(.white)
                            .cornerRadius(20)
                        }
                    }
                    .disabled(!isValidIP(ipAddress) || folderName.isEmpty)
                    .padding(.horizontal)
                    
                    Spacer()
                }
                .padding(.top, 40)
            }
            .navigationBarHidden(true)
            .sheet(isPresented: $showingScanner) {
                QRCodeScannerView { scannedCode in
                    showingScanner = false
                    ipAddress = scannedCode // Update the IP field
                }
            }
        }
    }
    
    func isValidIP(_ ip: String) -> Bool {
        let parts = ip.split(separator: ".")
        guard parts.count == 4 else { return false }
        
        return parts.allSatisfy { part in
            guard let number = Int(part) else { return false }
            return number >= 0 && number <= 255
        }
    }
}
