import SwiftUI

struct HomeView: View {
    @StateObject private var viewModel = ConnectionViewModel()
    @State private var showingScanner = false
    @State private var isShowingInputs = false
    @State private var navigateToSyncView = false
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    var body: some View {
        NavigationStack {
            ZStack {
                AnimatedBackgroundView(gradientColors: gradientColors)
                    .ignoresSafeArea()
                
                ScrollView {
                    VStack(spacing: 30) {
                        Spacer().frame(height: 40)
                        
                        VStack(spacing: 16) {
                            AnimatedLogo(isShowingInputs: isShowingInputs)
                            AppTitle(isShowingInputs: isShowingInputs)
                        }
                        .padding(.bottom, 25)
                        
                        VStack(spacing: 18) {
                            HStack {
                                CustomInputField(
                                    icon: "network",
                                    placeholder: "IP Address",
                                    text: $viewModel.credentials.ipAddress,
                                    width: 250,
                                    showError: viewModel.errorState.isShowing
                                )
                                
                                QRScannerButton(showingScanner: $showingScanner)
                            }
                            
                            CustomInputField(
                                icon: "folder",
                                placeholder: "Folder Name",
                                text: $viewModel.credentials.folderName,
                                width: 320,
                                showError: viewModel.errorState.isShowing
                            )
                        }
                        .opacity(isShowingInputs ? 1 : 0)
                        .offset(y: isShowingInputs ? 0 : 20)
                        
                        NavigationLink(
                            destination: SyncView(),
                            isActive: $navigateToSyncView
                        ) {
                            ConnectButton(
                                isConnecting: viewModel.isConnecting,
                                connectionProgress: viewModel.connectionProgress,
                                isEnabled: viewModel.credentials.isValid,
                                action: {
                                    if viewModel.credentials.isValid {
                                        viewModel.connect()
                                        navigateToSyncView = true
                                    }
                                }
                            )
                        }
                        .buttonStyle(PlainButtonStyle())  // Remove default navigation button styling
                        .opacity(isShowingInputs ? 1 : 0)
                        .offset(y: isShowingInputs ? 0 : 20)
                        
                        if viewModel.errorState.isShowing {
                            ErrorView(message: viewModel.errorState.message)
                        }
                        
                        Spacer()
                    }
                    .padding(.top, 20)
                }
            }
            .frame(minWidth: 400, minHeight: 600)
            .onAppear {
                withAnimation(.easeOut(duration: 0.8).delay(0.2)) {
                    isShowingInputs = true
                }
            }
        }
    }
}

#Preview() {
    HomeView()
}
