import SwiftUI

struct SyncView: View {
    @StateObject private var viewModel = SyncViewModel()
    @State private var navigateToDoneView = false
    @State private var showError = false
    @State private var errorMessage = ""
    
    var body: some View {
        NavigationStack {
            ZStack {
                GeometryReader { geometry in
                    ZStack {
                        BackgroundView(colors: SyncViewStyle.gradientColors)
                        
                        ForEach(viewModel.particles) { particle in
                            ParticleView(particle: particle)
                        }
                        
                        VStack {
                            titleView
                            Spacer()
                            ProgressCircleView(
                                viewModel: viewModel,
                                size: geometry.size.width
                            )
                            .frame(height: geometry.size.height * 0.4)
                            Spacer()
                            AnimatedPhraseView()
                                .frame(height: 40)
                                .padding(.horizontal)
                        }
                        .navigationBarBackButtonHidden(true)
                        .padding(.horizontal)
                    }
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .onAppear {
                    viewModel.startAnimations()
                    checkAndRequestPermissions { granted, selectedPath in
                        if granted {
                            do {
                                let x = try appleSync(
                                    host: "192.168.143.26",
                                    port: 1234,
                                    localDir: "files",  // Using relative path
                                    remoteDir: "files/"
                                )
                            } catch {
                                errorMessage = error.localizedDescription
                                showError = true
                            }
                        } else {
                            errorMessage = "Required permissions not granted"
                            showError = true
                        }
                    }
                }
                .alert("Error", isPresented: $showError) {
                    Button("OK", role: .cancel) { }
                } message: {
                    Text(errorMessage)
                }
            }
            .onChange(of: viewModel.isLoadingComplete) { isComplete in
                if isComplete {
                    navigateToDoneView = true
                }
            }
            .navigationDestination(isPresented: $navigateToDoneView) {
                DoneView()
            }
        }
    }
    
    private var titleView: some View {
        Text("Let the Magic Begin...")
            .font(.system(
                size: SyncViewStyle.Text.titleSize,
                weight: .bold
            ))
            .foregroundStyle(
                LinearGradient(
                    colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .shadow(color: .blue.opacity(0.3), radius: 10)
            .overlay(titleShine)
            .padding(.top, 20)
    }
    
    private var titleShine: some View {
        Rectangle()
            .fill(
                LinearGradient(
                    colors: [.clear, .white.opacity(0.2), .clear],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .offset(x: -200)
            .rotationEffect(.degrees(30))
            .mask(
                Text("Let the Magic Begin...")
                    .font(.system(
                        size: SyncViewStyle.Text.titleSize,
                        weight: .bold
                    ))
            )
            .animation(
                .linear(duration: 2)
                .repeatForever(autoreverses: false),
                value: viewModel.rotation
            )
    }
    
    func checkAndRequestPermissions(completion: @escaping (Bool, String?) -> Void) {
        let netManager = NetworkManager.shared
        netManager.requestPermission { networkGranted in
            FileManager.default.requestFileSystemPermission { fileSystemGranted, selectedPath in
                DispatchQueue.main.async {
                    completion(networkGranted && fileSystemGranted, selectedPath)
                }
            }
        }
    }
}

// File system permission helper
extension FileManager {
    func requestFileSystemPermission(completion: @escaping (Bool, String?) -> Void) {
        if Thread.isMainThread {
            showOpenPanel(completion: completion)
        } else {
            DispatchQueue.main.async {
                self.showOpenPanel(completion: completion)
            }
        }
    }
    
    private func showOpenPanel(completion: @escaping (Bool, String?) -> Void) {
        let openPanel = NSOpenPanel()
        openPanel.prompt = "Select Directory"
        openPanel.allowsMultipleSelection = false
        openPanel.canChooseDirectories = true
        openPanel.canChooseFiles = false
        
        openPanel.begin { response in
            DispatchQueue.main.async {
                if response == .OK {
                    // Create 'files' directory if it doesn't exist
                    if let selectedURL = openPanel.url {
                        let filesURL = selectedURL.appendingPathComponent("files")
                        do {
                            try self.createDirectory(at: filesURL, withIntermediateDirectories: true)
                            FileManager.default.changeCurrentDirectoryPath(selectedURL.path)
                            completion(true, selectedURL.path)
                        } catch {
                            completion(false, nil)
                        }
                    } else {
                        completion(false, nil)
                    }
                } else {
                    completion(false, nil)
                }
            }
        }
    }
}
// Network permission helper
class NetworkManager {
    static let shared = NetworkManager()
    
    func requestPermission(completion: @escaping (Bool) -> Void) {
        // On macOS 11 and later, you need to request local network permission
        if #available(macOS 11.0, *) {
            let session = URLSession(configuration: .default)
            let task = session.dataTask(with: URL(string: "http://localhost:1234")!) { _, _, _ in
                // This will trigger the permission prompt
                completion(true)
            }
            task.resume()
        } else {
            completion(true)
        }
    }
}
